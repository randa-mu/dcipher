//! Module for pedersen commitments over elliptic curves.

use ark_ec::CurveGroup;
use ark_std::UniformRand;
use rand::RngCore;

#[derive(Clone, Debug)]
pub struct PedersenCommitment<CG: CurveGroup> {
    pub opening: CG::ScalarField,
    pub commitment: CG,
}

/// Create a commitment using a randomly opening r.
/// In most cases, g and h must be distinct, independently picked generators where log_g h is unknown.
pub fn commit_rand<CG: CurveGroup, R: RngCore>(
    s: &CG::ScalarField,
    g: &CG,
    h: &CG,
    rng: &mut R,
) -> PedersenCommitment<CG> {
    let r = CG::ScalarField::rand(rng);

    let c = commit(s, &r, g, h);
    PedersenCommitment {
        opening: r,
        commitment: c,
    }
}

/// Create a Pedersen commitment deterministically using secret `s` and opening `r`.
/// In most cases, g and h must be distinct, independently picked generators where log_g h is unknown.
#[inline]
pub fn commit<CG: CurveGroup>(s: &CG::ScalarField, r: &CG::ScalarField, g: &CG, h: &CG) -> CG {
    *g * s + *h * r
}

/// Try to open a Pedersen commitment using a secret `s` and an opening scalar `r`.
/// Outputs true if the commitment is valid.
pub fn open<CG: CurveGroup>(
    s: &CG::ScalarField,
    opening: &CG::ScalarField,
    commitment: &CG,
    g: &CG,
    h: &CG,
) -> bool {
    *g * s + *h * opening == *commitment
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::{Group, pairing::Pairing};
    use ark_std::UniformRand;
    use utils::hash_to_curve::HashToCurve;

    #[test]
    fn consistency() {
        use ark_bn254::Bn254;
        let g = <Bn254 as Pairing>::G1::generator();
        let h = ark_bn254::G1Projective::hash_to_curve(b"PEDERSEN_H", b"TEST_DST_PEDERSEN_H");

        let mut rng = rand::thread_rng();
        let s = <Bn254 as Pairing>::ScalarField::rand(&mut rng);
        let PedersenCommitment {
            opening,
            commitment,
        } = commit_rand(&s, &g, &h, &mut rng);
        assert!(open(&s, &opening, &commitment, &g, &h));
    }
}
