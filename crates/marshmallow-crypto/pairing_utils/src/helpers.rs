use ark_ec::short_weierstrass::SWCurveConfig;

// https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-10.html#name-the-sgn0-function-2
// sgn0(x)
//
// Input: x, an element of GF(p^m).
// Output: 0 or 1.
//
pub(crate) fn sgn0<C: SWCurveConfig>(x: &C::BaseField) -> u8 {
    use ark_ff::{BigInteger, Field, PrimeField, Zero};

    let mut sign = 0;
    let mut zero = 1;
    for x_i in x.to_base_prime_field_elements() {
        let x_big = x_i.into_bigint();
        let sign_i = x_big.is_odd() as u8;
        let zero_i = x_i.is_zero() as u8;
        sign |= zero & sign_i;
        zero &= zero_i;
    }

    sign
}

/// Safe u32 to usize conversion on 32 bits and 64 bits platform
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[inline(always)]
pub(crate) const fn u32_to_usize(x: u32) -> usize {
    x as usize
}

/// Safe usize to u64 conversion on 32 bits and 64 bits platform
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[inline(always)]
#[allow(unused)]
pub(crate) const fn usize_to_u64(x: usize) -> u64 {
    x as u64
}

#[cfg(test)]
mod tests {
    mod bn254 {
        use ark_bn254::{Fq, Fq2};
        use rstest::*;

        use crate::helpers::sgn0;

        #[rstest]
        #[case(
            "17156762235442756890920543645786286404230753770466866674796434213025306755405",
            1
        )]
        #[case(
            "4168410880637855592787239578292129080744202471457696158726184257879218225221",
            1
        )]
        #[case(
            "9030550780579653150827548084928161235686401797305419500542003300201384284272",
            0
        )]
        #[case(
            "15654372863014602362061971772268178795161604642483138642096325789146141599972",
            0
        )]
        fn test_sgn0_bn254_g1(#[case] v: Fq, #[case] sign: u8) {
            assert_eq!(sgn0::<ark_bn254::g1::Config>(&v), sign)
        }

        #[rstest]
        #[case(
            "10223329234512403642177962733752475807273341614325302860723640962008089447",
            "16924605908551183949327845490510788619153940159315670751924447843621807232365",
            1
        )]
        #[case(
            "11368508181338748143154631470747420928705114614697721228010318086630263364431",
            "6331612496336989479385514459513634728788605833414226025170255718908601450990",
            1
        )]
        #[case(
            "2329673461211016333353911492796292867368236083477350736067628541250012592722",
            "1430975841972814014865627436982012633531907381111557949191337086719962383854",
            0
        )]
        #[case(
            "5949409610196079762171855895948799066719635580038444574227558119677970612924",
            "4846944208477993641824817069605929202924680158204846930121747143697167802107",
            0
        )]
        fn test_sgn0_bn254_g2(#[case] c0: Fq, #[case] c1: Fq, #[case] sign: u8) {
            let v = Fq2 { c0, c1 };
            assert_eq!(sgn0::<ark_bn254::g2::Config>(&v), sign)
        }
    }
}
