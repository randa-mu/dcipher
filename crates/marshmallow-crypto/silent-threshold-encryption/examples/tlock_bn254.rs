use anyhow::Result;
use ark_ec::pairing::{Pairing, PairingOutput};
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ec::AffineRepr;
use ark_ff::field_hashers::HashToField;
use ark_ff::{field_hashers::DefaultFieldHasher, UniformRand};
use ark_poly::univariate::DensePolynomial;
use ark_serialize::CanonicalSerializeHashExt;
use ark_std::{rand::Rng, Zero};
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use sha3::{digest::generic_array::GenericArray, Keccak256};
use silent_threshold_encryption::{
    decryption::agg_dec,
    encryption::{encrypt_with_gamma_g2, Ciphertext},
    kzg::{UniversalParams, KZG10},
    setup::{AggregateKey, SecretKey},
};
use std::{thread::sleep, time::Duration};

type E = ark_bn254::Bn254;
type G2 = <E as Pairing>::G2;
type UniPoly254 = DensePolynomial<<E as Pairing>::ScalarField>;

struct TlockCiphertext {
    silent_ct: Ciphertext<E>,
    ct: Vec<u8>,
    nonce: Vec<u8>,
}

fn round_to_gamma_g2(round: u64) -> <E as Pairing>::G2 {
    hash_to_g2(&round.to_be_bytes())
}

fn tlock_encrypt(
    plaintext: &[u8],
    round: u64,
    threshold: usize,
    ak: &AggregateKey<E>,
    params: &UniversalParams<E>,
) -> TlockCiphertext {
    println!(
        "Encrypting message `{}` for decryption at time {round}\n\n",
        std::str::from_utf8(plaintext).unwrap()
    );

    // Gamma is the hash of the current round number
    let gamma = round_to_gamma_g2(round);

    // Get ciphertext using the entire quorum with a threshold of 5
    let rand_pairing = PairingOutput::<E>::rand(&mut OsRng);
    let chacha_key = rand_pairing.hash::<Keccak256>();
    let cipher = ChaCha20Poly1305::new(&chacha_key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let chacha_ct = cipher.encrypt(&nonce, plaintext).unwrap();
    let tlock_ct = encrypt_with_gamma_g2::<E>(ak, threshold, rand_pairing, params, &gamma);

    TlockCiphertext {
        silent_ct: tlock_ct,
        ct: chacha_ct,
        nonce: nonce.to_vec(),
    }
}

fn main() -> Result<()> {
    let mut rng = ark_std::test_rng();

    // Number of parties + one dummy party
    const N: usize = 31 + 1;
    const _: () = assert!(N != 0 && (N & (N - 1)) == 0); // N non-zero and power of 2

    // Setup() w/ n = M + 1. May need to be a power of t: "Henceforth, we will consider that M + 1 is a power of 2 and we set the subgroup H = {ω0, ω1, . . . , ωM} of roots of unity to be such that |H| = M + 1. M is the maximum number of decryptors participating in the system"
    // todo: understand why power of 2
    // Likely need to set a high number of parties in prevision since we somehow need to safely agree upon a tau and compute the pairings
    // Still a one-time setup, but then we can onboard new parties by somply computing a different aggregation key in PreProcess.
    let params = KZG10::<E, UniPoly254>::setup(N, &mut rng).unwrap();
    // params = pp = CRS = ([\tau^1]_{1,2}, ..., [\tau^n]_{1,2})

    let mut sk = Vec::with_capacity(N);
    let mut pk = Vec::with_capacity(N);

    // Dummy party with sk0 = 1, pk0 = ([1]_1 = G, hint_i = HintGen(CRS, sk_i, i, M))
    sk.push(SecretKey::<E>::new(&mut rng));
    sk[0].nullify(); // sk0 = 1
    pk.push(sk[0].get_pk(0, &params, N)?);

    // Simulate the key generation for the n parties
    for i in 1..N {
        sk.push(SecretKey::<E>::new(&mut rng));
        pk.push(sk[i].get_pk(i, &params, N)?);
    }

    // Publish the public keys and hints to other parties
    // not sure what can of model is assumed, likely a verifiable broadcast from the games

    // This is basically preprocess, compute the sums for ak, ek. Rest of parameters are already in hints, i.e., pks
    let agg_key = AggregateKey::<E>::new(pk, &params);
    // Here, however, an encryptor could choose to exclude parties / add their own parties other than the LoE
    // Likely better to just publish the public key and encryptor computes it themselves

    // There is a polynomial evaluated at a point \tau, \tau^2, \tau^3
    // The coefficients of the polynomial is interpolated on-the-fly with each of the secret keys by computing \sum sk_i L_i(\tau) with L_i lagrange coefficients
    // cool, but can we decrypt with a different set of signers??
    // Yes: you do the aggregation for all the signers, and can decrypt if you provide a threshold number of partial sigs

    let threshold = 5; // do not account for dummy party
    let round = 5;
    let plaintext = format!("This ciphertext will be decrypted at round {}", round);
    let ct_tlock = tlock_encrypt(plaintext.as_bytes(), round, threshold, &agg_key, &params);

    let mut current_round = 1u64;
    loop {
        let gamma_g2 = round_to_gamma_g2(current_round);

        let mut partial_decs = Vec::new();
        let mut parties = Vec::new();
        let mut selector = [false; N];
        for _ in 0..threshold {
            // get unique random party
            let party = loop {
                let p = OsRng.gen_range(1..sk.len());
                if !selector[p] {
                    break p;
                }
            };

            partial_decs.push(sk[party].partial_decryption_gamma_g2(&gamma_g2));
            parties.push(party);
            selector[party] = true;
        }

        // We now have t partial decs, try to decrypt
        // Executed by whatever party that wants to decrypt tlock ciphertext
        let dec_key = agg_dec(
            &partial_decs,
            &parties,
            &ct_tlock.silent_ct,
            &agg_key,
            &params,
        );
        if let Ok(dec_key) = dec_key {
            println!("Ciphertext successfully decrypted at round {current_round}:");

            let chacha_key = dec_key.hash::<Keccak256>();
            let cipher = ChaCha20Poly1305::new(&chacha_key);
            let plaintext = cipher
                .decrypt(
                    GenericArray::from_slice(&ct_tlock.nonce),
                    ct_tlock.ct.as_ref(),
                )
                .expect("chacha ciphertext was altered");
            // Decryption should always work once the decryption key is obtained
            // due to correctness
            println!("{}", std::str::from_utf8(&plaintext).unwrap());
        } else {
            println!("Failed to decrypt ciphertext at round {current_round}");
        }

        sleep(Duration::from_millis(500));
        current_round += 1;
    }
}

use ark_bn254::{Fq, Fq2};
use ark_ff::Field;
use std::str::FromStr;

fn hash_to_g2(message: &[u8]) -> G2 {
    // Own params, check it
    const Z: Fq2 = Fq2::ONE;

    // 1. c1 = g(Z)
    let c1 = Fq2 {
        c0: Fq::from_str(
            "19485874751759354771024239261021720505790618469301721065564631296452457478374",
        )
        .unwrap(),
        c1: Fq::from_str(
            "266929791119991161246907387137283842545076965332900288569378510910307636690",
        )
        .unwrap(),
    };

    // 2. c2 = -Z / 2
    let c2 = Fq2 {
        c0: Fq::from_str(
            "10944121435919637611123202872628637544348155578648911831344518947322613104291",
        )
        .unwrap(),
        c1: Fq::ZERO,
    };

    // 3. c3 = sqrt(-g(Z) * (3 * Z^2 + 4 * A))     # sgn0(c3) MUST equal 0
    let c3 = Fq2 {
        c0: Fq::from_str(
            "2896050631867192331397261833972217924632033787908606332265566319765989907291",
        )
        .unwrap(),
        c1: Fq::from_str(
            "69234539592135073670822051309638369246835028322499721100120497037563571475",
        )
        .unwrap(),
    };

    // 4. c4 = -4 * g(Z) / (3 * Z^2 + 4 * A)
    let c4 = Fq2 {
        c0: Fq::from_str(
            "10499238450719652342378357227399831140106360636427411350395554762472100376473",
        )
        .unwrap(),
        c1: Fq::from_str(
            "6940174569119770192419592065569379906172001098655407502803841283667998553941",
        )
        .unwrap(),
    };

    let htf = <DefaultFieldHasher<sha3::Keccak256> as HashToField<Fq2>>::new(b"somedst");
    let m: Vec<Fq2> = htf.hash_to_field(message, 2);
    let q0 = svdw_map_to_curve::<ark_bn254::g2::Config>(m[0], [c1, c2, c3, c4], Z);
    let q1 = svdw_map_to_curve::<ark_bn254::g2::Config>(m[1], [c1, c2, c3, c4], Z);
    let r: Affine<ark_bn254::g2::Config> = (q0 + q1).into();

    let p = r.clear_cofactor();
    debug_assert!(p.is_in_correct_subgroup_assuming_on_curve());
    p.into()
}

fn svdw_map_to_curve<C: SWCurveConfig>(
    u: C::BaseField,
    c: [C::BaseField; 4],
    z: C::BaseField,
) -> Affine<C> {
    #![allow(clippy::assign_op_pattern)]

    assert!(C::COEFF_A.is_zero());
    let is_square = |f: C::BaseField| f.legendre().is_qr();

    // https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-10.html#section-f.1
    //    1. c1 = g(Z)
    let c1 = c[0];
    //    2. c2 = -Z / 2
    let c2 = c[1];
    //    3. c3 = sqrt(-g(Z) * (3 * Z^2 + 4 * A))     # sgn0(c3) MUST equal 0
    let c3 = c[2];
    //    4. c4 = -4 * g(Z) / (3 * Z^2 + 4 * A)
    let c4 = c[3];

    //    1.  tv1 = u^2
    let mut tv1 = u.square();
    //    2.  tv1 = tv1 * c1
    tv1 = tv1 * c1;
    //    3.  tv2 = 1 + tv1
    let tv2 = C::BaseField::ONE + tv1;
    //    4.  tv1 = 1 - tv1
    tv1 = C::BaseField::ONE - tv1;
    //    5.  tv3 = tv1 * tv2
    let mut tv3 = tv1 * tv2;
    //    6.  tv3 = inv0(tv3)
    tv3 = tv3.inverse().unwrap();
    //    7.  tv4 = u * tv1
    let mut tv4 = u * tv1;
    //    8.  tv4 = tv4 * tv3
    tv4 = tv4 * tv3;
    //    9.  tv4 = tv4 * c3
    tv4 = tv4 * c3;
    //    10.  x1 = c2 - tv4
    let x1 = c2 - tv4;
    //    11. gx1 = x1^2
    let mut gx1 = x1.square();
    //    12. gx1 = gx1 + A
    //    gx1 = gx1 + C::COEFF_A; // a is 0 for used curves.

    //    13. gx1 = gx1 * x1
    gx1 = gx1 * x1;
    //    14. gx1 = gx1 + B
    gx1 = gx1 + C::COEFF_B;

    //    15.  e1 = is_square(gx1)
    let e1 = is_square(gx1);
    //    16.  x2 = c2 + tv4
    let x2 = c2 + tv4;
    //    17. gx2 = x2^2
    let mut gx2 = x2.square();
    //    18. gx2 = gx2 + A
    //    gx2 = gx2 + C::COEFF_A; // a is 0 for used curves.

    //    19. gx2 = gx2 * x2
    gx2 = gx2 * x2;
    //    20. gx2 = gx2 + B
    gx2 = gx2 + C::COEFF_B;
    //    21.  e2 = is_square(gx2) AND NOT e1
    let e2 = is_square(gx2) && !e1;
    //    22.  x3 = tv2^2
    let mut x3 = tv2.square();
    //    23.  x3 = x3 * tv3
    x3 = x3 * tv3;
    //    24.  x3 = x3^2
    x3 = x3.square();
    //    25.  x3 = x3 * c4
    x3 = x3 * c4;
    //    26.  x3 = x3 + Z
    x3 = x3 + z;

    // CMOV requires `subtle`, not supported by arkworks.
    //    27.  x = CMOV(x3, x1, e1)      # x = x1 if gx1 is square, else x = x3
    let mut x = if e1 { x1 } else { x3 };
    //    28.  x = CMOV(x, x2, e2)       # x = x2 if gx2 is square and gx1 is not
    if e2 {
        x = x2;
    }
    //    29.  gx = x^2
    let mut gx = x.square();
    //    30.  gx = gx + A
    //    gx = gx + C::COEFF_A; // a is 0 for used curves.

    //    31.  gx = gx * x
    gx = gx * x;
    //    32.  gx = gx + B
    gx = gx + C::COEFF_B;
    //    33.   y = sqrt(gx)
    let mut y = gx.sqrt().unwrap();
    //    34.  e3 = sgn0(u) == sgn0(y)
    let e3 = sgn0::<C>(u) == sgn0::<C>(y);
    //    35. y = CMOV(-y, y, e3)       # Select correct sign of y
    if !e3 {
        y = -y;
    }

    let point = Affine::new_unchecked(x, y);
    debug_assert!(point.is_on_curve());

    point
}

// https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-10.html#name-the-sgn0-function-2
// sgn0_m_eq_1(x)
//
// Input: x, an element of GF(p).
// Output: 0 or 1.
//
// Steps:
// 1. return x mod 2
//
fn sgn0<C: SWCurveConfig>(x: C::BaseField) -> u8 {
    let mut sign = 0;
    let mut zero = 1;
    for x_i in x.to_base_prime_field_elements() {
        let sign_i = Into::<num_bigint::BigUint>::into(x_i).bit(0) as u8;
        let zero_i = x_i.is_zero() as u8;
        sign |= zero & sign_i;
        zero &= zero_i;
    }

    sign
}
