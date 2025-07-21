use ark_ec::{
    pairing::{Pairing, PairingOutput},
    Group, VariableBaseMSM,
};
use ark_poly::{
    univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, Polynomial,
    Radix2EvaluationDomain,
};
use ark_std::{One, Zero};
use std::{
    cmp::Ordering,
    collections::HashSet,
    ops::{Div, Neg},
};

use crate::{
    encryption::{Ciphertext, CiphertextCCA2},
    errors::{DecryptionError, VerificationError},
    kzg::{UniversalParams, KZG10},
    setup::AggregateKey,
    utils::interp_mostly_zero,
};

fn get_missing_parties(parties: &[usize], n: usize) -> Vec<usize> {
    let parties: HashSet<&usize> = HashSet::from_iter(parties);
    (0..n).filter(|i| !parties.contains(i)).collect()
}

fn verify_partial_decryptions<E: Pairing>(
    partial_decryptions: &[E::G2],
    parties: &[usize],
    ct: &Ciphertext<E>,
    agg_key: &AggregateKey<E>,
) -> Result<(), VerificationError> {
    // Verify the partial decryptions
    let gamma_g2_affine = ct.gamma_g2.into();
    let g1_affine = E::G1::generator().into();
    for (&i, &sigma_i) in parties.iter().zip(partial_decryptions) {
        // PartVerify(ct, \sigma_i, pk_i)
        // Likely need to also ensure that sigma_i lies on the correct group or something?
        if !E::multi_pairing(
            [agg_key.pk[i].bls_pk.neg().into(), g1_affine],
            [gamma_g2_affine, sigma_i.into()],
        )
        .is_zero()
        // is_zero(): Checks if pairing output is equal to identity
        {
            Err(VerificationError::InvalidPartialSig(i))? // Verification failed
        }
    }

    Ok(())
}

pub fn agg_dec<E: Pairing>(
    mut partial_decryptions: &[E::G2],
    mut parties: &[usize],
    ct: &Ciphertext<E>,
    agg_key: &AggregateKey<E>,
    params: &UniversalParams<E>,
) -> Result<PairingOutput<E>, DecryptionError> {
    // Arrays of different length
    if partial_decryptions.len() != parties.len() {
        Err(DecryptionError::InconsistentPartiesArraysLength(
            partial_decryptions.len(),
            parties.len(),
        ))?
    }

    // Not enough partial decryptions / parties
    match parties.len().cmp(&ct.t) {
        Ordering::Less => Err(DecryptionError::NotEnoughParties(parties.len(), ct.t))?,
        Ordering::Greater => {
            // If more than t, pick the first t partial decryptions
            partial_decryptions = &partial_decryptions[..ct.t];
            parties = &parties[..ct.t];
        }
        std::cmp::Ordering::Equal => {}
    }

    // Invalid partial decryptions
    if let Err(e) = verify_partial_decryptions(partial_decryptions, parties, ct, agg_key) {
        Err(DecryptionError::Verification(e))?
    }

    let n = agg_key.pk.len();
    let missing_parties = get_missing_parties(parties, n); // includes dummy party

    let domain = Radix2EvaluationDomain::<E::ScalarField>::new(n)
        .ok_or(DecryptionError::EvaluationDomain)?;
    let domain_elements: Vec<E::ScalarField> = domain.elements().collect();

    // points is where B is set to zero
    // parties is the set of parties who have signed
    let points = missing_parties
        .iter()
        .map(|&i| domain_elements[i])
        .collect::<Vec<_>>();

    // DecAggr, Step 1.
    // Interpolate polynomial with points = (ω^0, 1), (ω^i, 0)_{i \in [n] \setminus parties}
    let b = interp_mostly_zero(E::ScalarField::one(), &points);

    // b_evals = (B(ω^0), ..., B(ω^n))
    let b_evals = domain.fft(&b.coeffs);
    let b_evals_parties: Vec<E::ScalarField> = parties.iter().map(|&i| b_evals[i]).collect();

    debug_assert!(b.degree() == points.len() - 1);
    debug_assert!(b.evaluate(&domain_elements[0]) == E::ScalarField::one());

    // commit to b in g2
    let b_g2: E::G2 = KZG10::<E, DensePolynomial<E::ScalarField>>::commit_g2(params, &b)?.into();

    // DecAggr, Step 8.
    // B(X) - 1
    let mut bminus1 = b.clone();
    bminus1.coeffs[0] -= E::ScalarField::one();

    debug_assert!(bminus1.evaluate(&domain_elements[0]) == E::ScalarField::zero());

    let xminus1 =
        DensePolynomial::from_coefficients_vec(vec![-domain_elements[0], E::ScalarField::one()]);
    // Q0(X) = (B(X) - 1)/(X - domain_elements[0])
    let q0 = bminus1.div(&xminus1);

    // Q0 = [Q0(τ)]_1
    let q0_g1: E::G1 = KZG10::<E, DensePolynomial<E::ScalarField>>::commit_g1(params, &q0)?.into();

    // DecAggr, Step 7.
    // insert t 0s at the beginning of bhat.coeffs
    // this way, we start evaluating the polynomial with τ^t
    let mut bhat_coeffs = vec![E::ScalarField::zero(); ct.t];
    bhat_coeffs.append(&mut b.coeffs.clone());
    let bhat = DensePolynomial::from_coefficients_vec(bhat_coeffs);
    debug_assert_eq!(bhat.degree(), n - 1);

    // \hat{B} = [τ^t B(τ)]_1
    let bhat_g1: E::G1 =
        KZG10::<E, DensePolynomial<E::ScalarField>>::commit_g1(params, &bhat)?.into();

    let n_inv = E::ScalarField::one() / E::ScalarField::from((n) as u32);

    // DecAggr, Step 7.
    // compute the aggregate public key aPK
    let bases: Vec<<E as Pairing>::G1Affine> = parties
        .iter()
        .map(|&i| agg_key.pk[i].bls_pk.into())
        .collect();
    // aPK = 1/n ([1]_1 * 1 + \sum_{i \in S} B(ω^i) pk_i)
    let mut apk = params.powers_of_g[0] // dummy party's contribution
        + E::G1::msm(bases.as_slice(), b_evals_parties.as_slice()).unwrap();
    apk *= n_inv;

    // DecAggr, Step 6.
    // compute aggregated sigma
    let bases: Vec<E::G2Affine> = partial_decryptions
        .iter()
        .map(|&sigma| sigma.into())
        .collect();
    // sigma = 1/n (1 * sigma_1 + \sum_{i \in S} B(ω^i) sigma_i)
    // Note that sigma_1 == ct1 == [\gamma]_2
    let mut sigma = ct.gamma_g2 // dummy party's contribution
        + E::G2::msm(bases.as_slice(), b_evals_parties.as_slice()).unwrap();
    sigma *= n_inv;

    // DecAggr, Step 4 - Compute Qx
    let bases: Vec<E::G1Affine> = parties
        .iter()
        .map(|&i| agg_key.pk[i].sk_li_by_tau.into())
        .collect();
    // Qx = [Qx(τ)]_1
    let qx = agg_key.pk[0].sk_li_by_tau // dummy party's contribution
        + E::G1::msm(bases.as_slice(), b_evals_parties.as_slice()).unwrap();

    // DecAggr, Step 3 - Compute QZ
    let bases: Vec<E::G1Affine> = parties
        .iter()
        .map(|&i| agg_key.agg_sk_li_by_z[i].into())
        .collect();
    // QZ = [QZ(τ)]_1
    let qz = agg_key.agg_sk_li_by_z[0] // dummy party's contribution
        + E::G1::msm(bases.as_slice(), b_evals_parties.as_slice()).unwrap();

    // DecAggr, Step 3 - Compute \hat{Qx}
    let bases: Vec<E::G1Affine> = parties
        .iter()
        .map(|&i| agg_key.pk[i].sk_li_minus0.into())
        .collect();
    // \hat{Qx} = [Qx(τ)*τ]_1
    let qhatx = agg_key.pk[0].sk_li_minus0 // dummy party's contribution
        + E::G1::msm(bases.as_slice(), b_evals_parties.as_slice()).unwrap();

    // e(w1||sa1, sa2||w2)
    let w1 = [
        apk.neg(),
        qz.neg(),
        qx.neg(),
        qhatx,
        bhat_g1.neg(),
        q0_g1.neg(),
    ];
    let w2 = [b_g2, sigma];

    let mut enc_key_lhs = w1.to_vec();
    enc_key_lhs.append(&mut ct.sa1.to_vec());

    let mut enc_key_rhs = ct.sa2.to_vec();
    enc_key_rhs.append(&mut w2.to_vec());

    let enc_key = E::multi_pairing(enc_key_lhs, enc_key_rhs);

    // If t partial decryptions are valid, i.e., PartVerify(ct, \sigma_i, pk_i) = 1 for i \in S, |S| = t,
    // and isValid(CRS, hint_i, pk_i) = 1
    // the output is correct
    Ok(ct.enc_key - enc_key)
}

pub fn agg_dec_cca2<E: Pairing>(
    partial_decryptions: &[E::G2], //insert 0 if a party did not respond or verification failed
    parties: &[usize],
    ct_cca2: &CiphertextCCA2<E>,
    agg_key: &AggregateKey<E>,
    params: &UniversalParams<E>,
) -> Result<PairingOutput<E>, DecryptionError>
where
    CiphertextCCA2<E>: Into<Ciphertext<E>>,
{
    let ct = ct_cca2.clone().into();
    if ct_cca2.proof.verify_with_ciphertext(ct_cca2).is_ok() {
        agg_dec(partial_decryptions, parties, &ct, agg_key, params)
    } else {
        Err(DecryptionError::InvalidCca2Proof)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        encryption::encrypt,
        kzg::KZG10,
        setup::{PublicKey, SecretKey},
    };
    use ark_poly::univariate::DensePolynomial;

    type E = ark_bls12_381::Bls12_381;
    type G2 = <E as Pairing>::G2;
    type UniPoly381 = DensePolynomial<<E as Pairing>::ScalarField>;

    #[test]
    fn test_decryption() {
        let mut rng = ark_std::test_rng();
        let n = 1 << 4; // actually n-1 total parties. one party is a dummy party that is always true
        let t: usize = n / 2;
        debug_assert!(t < n);

        let params = KZG10::<E, UniPoly381>::setup(n, &mut rng).unwrap();

        let mut sk: Vec<SecretKey<E>> = Vec::new();
        let mut pk: Vec<PublicKey<E>> = Vec::new();

        // create the dummy party's keys
        sk.push(SecretKey::<E>::new(&mut rng));
        sk[0].nullify();
        pk.push(sk[0].get_pk(0, &params, n).unwrap());

        for i in 1..n {
            sk.push(SecretKey::<E>::new(&mut rng));
            pk.push(sk[i].get_pk(i, &params, n).unwrap())
        }

        let agg_key = AggregateKey::<E>::new(pk, &params);
        let ct = encrypt::<E>(&agg_key, t, &params);

        // compute partial decryptions
        let parties: Vec<usize> = (1..=t).collect();
        let partial_decryptions: Vec<G2> = parties
            .iter()
            .map(|&i| sk[i].partial_decryption(&ct))
            .collect();

        // compute the decryption key
        let _dec_key = agg_dec(&partial_decryptions, &parties, &ct, &agg_key, &params);

        assert!(_dec_key.is_ok());
        // Message-less encryption, hence _dec_key should be zero
        assert!(_dec_key.unwrap().is_zero());
    }

    #[test]
    fn test_invalid_decryption() {
        let mut rng = ark_std::test_rng();
        let n = 1 << 4; // actually n-1 total parties. one party is a dummy party that is always true
        let t: usize = n / 2;
        debug_assert!(t < n);

        let params = KZG10::<E, UniPoly381>::setup(n, &mut rng).unwrap();

        let mut sk: Vec<SecretKey<E>> = Vec::new();
        let mut pk: Vec<PublicKey<E>> = Vec::new();

        // create the dummy party's keys
        sk.push(SecretKey::<E>::new(&mut rng));
        sk[0].nullify();
        pk.push(sk[0].get_pk(0, &params, n).unwrap());

        for i in 1..n {
            sk.push(SecretKey::<E>::new(&mut rng));
            pk.push(sk[i].get_pk(i, &params, n).unwrap())
        }

        let agg_key = AggregateKey::<E>::new(pk, &params);
        let ct = encrypt::<E>(&agg_key, t, &params);

        // compute partial decryptions
        let parties: Vec<usize> = (1..=t).collect();
        let mut partial_decryptions: Vec<G2> = parties
            .iter()
            .map(|&i| sk[i].partial_decryption(&ct))
            .collect();

        // corrupt partial decryption
        partial_decryptions[0] = partial_decryptions[0].double();

        // compute the decryption key
        let _dec_key = agg_dec(&partial_decryptions, &parties, &ct, &agg_key, &params);

        assert!(_dec_key.is_err());
    }
}
