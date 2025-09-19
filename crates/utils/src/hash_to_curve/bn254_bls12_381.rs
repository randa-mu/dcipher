/// Macro used to generate a [`CustomHashToCurve`] and [`HashToCurve`] implementation for a given
/// [`SWCurveConfig`], a default hash function, and a hash to curve function.
macro_rules! gen_hash_to_curve {
    ($cg_config:ty, $f:ident, $h:ty, $d:literal) => {
        impl CustomHashToCurve for ark_ec::short_weierstrass::Projective<$cg_config> {
            fn hash_to_curve_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
                message: &[u8],
                dst: &[u8],
            ) -> Self {
                $f::<H>(message, dst)
            }
        }

        impl HashToCurve for ark_ec::short_weierstrass::Projective<$cg_config> {
            const CIPHERSUITE: &'static str = $d;
            type DefaultInnerHash = $h;

            fn hash_to_curve(message: &[u8], dst: &[u8]) -> Self {
                $f::<Self::DefaultInnerHash>(message, dst)
            }
        }
    };
}

#[cfg(feature = "bn254")]
mod bn254 {
    use crate::hash_to_curve::{CustomHashToCurve, CustomPairingHashToCurve, HashToCurve};
    use ark_ff::{AdditiveGroup, MontFp};
    use digest::core_api::BlockSizeUser;
    use digest::FixedOutputReset;

    gen_hash_to_curve!(
        ark_bn254::g1::Config,
        bn254_hash_to_g1_custom,
        sha2::Sha256,
        "BN254G1_XMD:SHA-256_SVDW_RO_"
    );
    gen_hash_to_curve!(
        ark_bn254::g2::Config,
        bn254_hash_to_g2_custom,
        sha2::Sha256,
        "BN254G2_XMD:SHA-256_SVDW_RO_"
    );

    impl CustomPairingHashToCurve for ark_bn254::Bn254 {
        fn hash_to_g1_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
            message: &[u8],
            dst: &[u8],
        ) -> Self::G1 {
            Self::G1::hash_to_curve_custom::<H>(message, dst)
        }

        fn hash_to_g2_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
            message: &[u8],
            dst: &[u8],
        ) -> Self::G2 {
            Self::G2::hash_to_curve_custom::<H>(message, dst)
        }
    }

    /// Hash message into a point on the G1 group of the bn254 curve using a custom hash function and dst.
    fn bn254_hash_to_g1_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
        message: &[u8],
        dst: &[u8],
    ) -> ark_bn254::G1Projective {
        use ark_bn254::Fq;
        use ark_ec::short_weierstrass::Affine;
        use ark_ec::AffineRepr;
        use ark_ff::{
            field_hashers::{DefaultFieldHasher, HashToField},
            Field, MontFp,
        };

        // Same params as [gnark](https://github.com/Consensys/gnark-crypto/blob/6cf8884de6417ad6ac95e6304bed6468b3587378/ecc/bn254/hash_to_g1.go#L38), [kyber](https://github.com/drand/kyber/blob/94dae51d79b4b0c2d2a9b9cc382b864cf3537783/pairing/bn254/constants.go#L63)
        const Z: Fq = Fq::ONE;

        const C: [Fq; 4] = [
            MontFp!("4"),
            MontFp!(
                "10944121435919637611123202872628637544348155578648911831344518947322613104291"
            ),
            MontFp!("8815841940592487685674414971303048083897117035520822607866"),
            MontFp!("7296080957279758407415468581752425029565437052432607887563012631548408736189"),
        ];
        let htf = <DefaultFieldHasher<H, 128> as HashToField<Fq>>::new(dst);
        let m = htf.hash_to_field::<2>(message);
        let q0 = super::svdw::map_to_curve_const_a_zero::<ark_bn254::g1::Config>(m[0], C, Z);
        let q1 = super::svdw::map_to_curve_const_a_zero::<ark_bn254::g1::Config>(m[1], C, Z);
        let r: Affine<ark_bn254::g1::Config> = (q0 + q1).into();

        let p = r.clear_cofactor();
        debug_assert!(p.is_on_curve());
        debug_assert!(p.is_in_correct_subgroup_assuming_on_curve());
        p.into_group()
    }

    /// Hash to a point on the G2 group of the bn254 curve using a custom hash function and dst.
    fn bn254_hash_to_g2_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
        message: &[u8],
        dst: &[u8],
    ) -> ark_bn254::G2Projective {
        use ark_bn254::{Fq, Fq2};
        use ark_ec::short_weierstrass::Affine;
        use ark_ec::AffineRepr;
        use ark_ff::{
            field_hashers::{DefaultFieldHasher, HashToField},
            Field, MontFp,
        };

        // Params computed using https://github.com/cfrg/draft-irtf-cfrg-hash-to-curve/blob/664b13592116cecc9e52fb192dcde0ade36f904e/poc/z_selection.sage#L42, same params as [gnark](https://github.com/Consensys/gnark-crypto/blob/6cf8884de6417ad6ac95e6304bed6468b3587378/ecc/bn254/hash_to_g2.go#L39)
        const Z: Fq2 = Fq2::ONE;

        // 1. c1 = g(Z)
        const C1: Fq2 = Fq2::new(
            MontFp!(
                "19485874751759354771024239261021720505790618469301721065564631296452457478374"
            ),
            MontFp!("266929791119991161246907387137283842545076965332900288569378510910307636690"),
        );

        // 2. c2 = -Z / 2
        const C2: Fq2 = Fq2::new(
            MontFp!(
                "10944121435919637611123202872628637544348155578648911831344518947322613104291"
            ),
            Fq::ZERO,
        );

        // 3. c3 = sqrt(-g(Z) * (3 * Z^2 + 4 * A))     # sgn0(c3) MUST equal 0
        const C3: Fq2 = Fq2::new(
            MontFp!(
                "18992192239972082890849143911285057164064277369389217330423471574879236301292"
            ),
            MontFp!(
                "21819008332247140148575583693947636719449476128975323941588917397607662637108"
            ),
        );

        // 4. c4 = -4 * g(Z) / (3 * Z^2 + 4 * A)
        const C4: Fq2 = Fq2::new(
            MontFp!(
                "10499238450719652342378357227399831140106360636427411350395554762472100376473"
            ),
            MontFp!("6940174569119770192419592065569379906172001098655407502803841283667998553941"),
        );

        const C: [Fq2; 4] = [C1, C2, C3, C4];

        let htf = <DefaultFieldHasher<H, 128> as HashToField<Fq2>>::new(dst);
        let m = htf.hash_to_field::<2>(message);
        let q0 = super::svdw::map_to_curve_const_a_zero::<ark_bn254::g2::Config>(m[0], C, Z);
        let q1 = super::svdw::map_to_curve_const_a_zero::<ark_bn254::g2::Config>(m[1], C, Z);
        let r = q0 + q1;

        let p: Affine<ark_bn254::g2::Config> = bn254_g2_clear_cofactor(r).into();
        debug_assert!(p.is_on_curve());
        debug_assert!(p.is_in_correct_subgroup_assuming_on_curve());
        p.into_group()
    }

    /// Clear cofactor based on http://cacr.uwaterloo.ca/techreports/2011/cacr2011-26.pdf, 6.1
    /// Adapted from: https://github.com/nikkolasg/bn254_hash2curve/blob/5995e36149b0119fa2a97dfcc00758729f00cc93/src/hash2g2.rs#L291
    fn bn254_g2_clear_cofactor(p: ark_bn254::G2Projective) -> ark_bn254::G2Projective {
        use ark_bn254::{Fq2, Fr};
        use ark_ec::AdditiveGroup;

        const SEED: Fr = MontFp!("4965661367192848881");
        const ENDO_U: Fq2 = Fq2::new(
            MontFp!(
                "21575463638280843010398324269430826099269044274347216827212613867836435027261"
            ),
            MontFp!(
                "10307601595873709700152284273816112264069230130616436755625194854815875713954"
            ),
        );

        const ENDO_V: Fq2 = Fq2::new(
            MontFp!("2821565182194536844548159561693502659359617185244120367078079554186484126554"),
            MontFp!("3505843767911556378687030309984248845540243509899259641013678093033130930403"),
        );

        fn psi(mut p: ark_bn254::G2Projective) -> ark_bn254::G2Projective {
            p.x.conjugate_in_place();
            p.x *= ENDO_U;

            p.y.conjugate_in_place();
            p.y *= ENDO_V;

            p.z.conjugate_in_place();
            p
        }

        let p0 = p * SEED;
        let p1 = psi(p0.double() + p0);
        let p2 = psi(psi(p0));
        let p3 = psi(psi(psi(p)));
        p0 + p1 + p2 + p3
    }
}

#[cfg(feature = "bls12-381")]
mod bls12_381 {
    use crate::hash_to_curve::{CustomHashToCurve, CustomPairingHashToCurve, HashToCurve};
    use digest::core_api::BlockSizeUser;
    use digest::FixedOutputReset;

    gen_hash_to_curve!(
        ark_bls12_381::g1::Config,
        bls12381_hash_to_g1_custom,
        sha2::Sha256,
        "BLS12_381G1_XMD:SHA-256_SVDW_RO_"
    );

    gen_hash_to_curve!(
        ark_bls12_381::g2::Config,
        bls12381_hash_to_g2_custom,
        sha2::Sha256,
        "BLS12_381G2_XMD:SHA-256_SVDW_RO_"
    );

    impl CustomPairingHashToCurve for ark_bls12_381::Bls12_381 {
        fn hash_to_g1_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
            message: &[u8],
            dst: &[u8],
        ) -> Self::G1 {
            Self::G1::hash_to_curve_custom::<H>(message, dst)
        }

        fn hash_to_g2_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
            message: &[u8],
            dst: &[u8],
        ) -> Self::G2 {
            Self::G2::hash_to_curve_custom::<H>(message, dst)
        }
    }

    fn bls12381_hash_to_g1_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
        message: &[u8],
        dst: &[u8],
    ) -> ark_bls12_381::G1Projective {
        use ark_bls12_381::{Config, G1Projective};
        use ark_ec::{
            bls12::Bls12Config,
            hashing::{
                curve_maps::wb::WBMap, map_to_curve_hasher::MapToCurveBasedHasher, HashToCurve,
            },
        };
        use ark_ff::field_hashers::DefaultFieldHasher;

        let hasher = MapToCurveBasedHasher::<
            G1Projective,
            DefaultFieldHasher<H, 128>,
            WBMap<<Config as Bls12Config>::G1Config>,
        >::new(dst)
        .unwrap();

        hasher.hash(message).unwrap().into()
    }

    fn bls12381_hash_to_g2_custom<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
        message: &[u8],
        dst: &[u8],
    ) -> ark_bls12_381::G2Projective {
        use ark_ec::hashing::{
            curve_maps::wb::WBMap, map_to_curve_hasher::MapToCurveBasedHasher, HashToCurve,
        };
        use ark_ff::field_hashers::DefaultFieldHasher;

        let hasher = MapToCurveBasedHasher::<
            ark_bls12_381::G2Projective,
            DefaultFieldHasher<H, 128>,
            WBMap<<ark_bls12_381::Config as ark_ec::bls12::Bls12Config>::G2Config>,
        >::new(dst)
        .unwrap();

        hasher.hash(message).unwrap().into()
    }
}

#[cfg(feature = "svdw")]
mod svdw {
    use ark_ec::short_weierstrass::{Projective, SWCurveConfig};

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

    pub(super) fn map_to_curve_const_a_zero<C: SWCurveConfig>(
        u: C::BaseField,
        c: [C::BaseField; 4],
        z: C::BaseField,
    ) -> Projective<C> {
        #![allow(clippy::assign_op_pattern)]
        use ark_ff::{Field, Zero};

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
        let e3 = sgn0::<C>(&u) == sgn0::<C>(&y);
        //    35. y = CMOV(-y, y, e3)       # Select correct sign of y
        if !e3 {
            y = -y;
        }

        Projective::new_unchecked(x, y, C::BaseField::ONE)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bn254")]
    mod bn254 {
        use crate::hash_to_curve::bn254_bls12_381::svdw::{map_to_curve_const_a_zero, sgn0};
        use crate::hash_to_curve::CustomPairingHashToCurve;
        use ark_bn254::{Fq, Fq2};
        use ark_ec::short_weierstrass::SWCurveConfig;
        use ark_ec::{short_weierstrass::Affine, CurveGroup};
        use ark_ff::BigInt;
        use ark_ff::Field;
        use ark_ff::MontFp;
        use ark_ff::PrimeField;
        use rstest::*;

        #[test]
        fn test_inv0_regression() {
            // ensure inv0() returns 0 on input 0.
            // this input works because the expression is inv0(tv1 * tv2), and tv1 * tv2 = 1 - (4u^2)^2
            // so with u = 2^-1, we have 4u^2 = 1, and thus tv1 * tv2 = 0.
            const Z: Fq = Fq::ONE;

            const C: [Fq; 4] = [
                MontFp!("4"),
                MontFp!(
                    "10944121435919637611123202872628637544348155578648911831344518947322613104291"
                ),
                MontFp!("8815841940592487685674414971303048083897117035520822607866"),
                MontFp!(
                    "7296080957279758407415468581752425029565437052432607887563012631548408736189"
                ),
            ];
            let u: Fq = MontFp!("2").inverse().unwrap();
            let r = map_to_curve_const_a_zero::<ark_bn254::g1::Config>(u, C, Z);
            assert_eq!(
                r.x,
                BigInt([
                    11389680472494603939,
                    14681934109093717318,
                    15863968012492123182,
                    1743499133401485332
                ])
                .into()
            );
            assert_eq!(
                r.y,
                BigInt([
                    5912843033038583732,
                    13018452156251680787,
                    18315244615680547370,
                    751716899383612316
                ])
                .into()
            );
        }

        fn hex64_to_g1_bn254<C: SWCurveConfig>(hex_str_x: &str, hex_str_y: &str) -> Affine<C>
        where
            C::BaseField: PrimeField,
        {
            let ser_x = hex::decode(hex_str_x).unwrap();
            let ser_y = hex::decode(hex_str_y).unwrap();

            let x = C::BaseField::from_be_bytes_mod_order(&ser_x[0..32]);
            let y = C::BaseField::from_be_bytes_mod_order(&ser_y[0..32]);

            Affine::new(x, y)
        }

        fn hex64_to_g2_bn254(hex_str_x: &str, hex_str_y: &str) -> ark_bn254::G2Affine {
            let ser_x = hex::decode(hex_str_x).unwrap();
            let ser_y = hex::decode(hex_str_y).unwrap();

            let x0 = ark_bn254::Fq::from_be_bytes_mod_order(&ser_x[0..32]);
            let x1 = ark_bn254::Fq::from_be_bytes_mod_order(&ser_x[32..64]);
            let x = ark_bn254::Fq2::new(x0, x1);

            let y0 = ark_bn254::Fq::from_be_bytes_mod_order(&ser_y[0..32]);
            let y1 = ark_bn254::Fq::from_be_bytes_mod_order(&ser_y[32..64]);
            let y = ark_bn254::Fq2::new(y0, y1);

            Affine::new(x, y)
        }

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

        // Taken from: https://github.com/drand/kyber/blob/94dae51d79b4b0c2d2a9b9cc382b864cf3537783/pairing/bn254/point_test.go#L12
        #[test]
        fn drand_compatibility_bn254_g1_hash_to_point() {
            const DST: &[u8] = b"domain_separation_tag_test_12345";
            let msg = b"The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

            let p = ark_bn254::Bn254::hash_to_g1_custom::<sha3::Keccak256>(msg, DST);
            assert_eq!(
                p,
                hex64_to_g1_bn254(
                    "13af4ace8febc1ec800f7d33d66868310516bce9cb1b7f7c68607f9ba6dba92c",
                    "1823b8f13feeb8dad6b152eb2bbefbe59452f9519c88230b55d0b699498db6f1"
                )
            );

            let msg =
                hex::decode("e0a05cbb37fd6c159732a8c57b981773f7480695328b674d8a9cc083377f1811")
                    .unwrap();
            let p = ark_bn254::Bn254::hash_to_g1_custom::<sha3::Keccak256>(&msg, DST);
            assert_eq!(
                p,
                hex64_to_g1_bn254(
                    "07abd743dc93dfa3a8ee4ab449b1657dc6232c589612b23a54ea461c7232101e",
                    "2533badbee56e8457731fc35bb7630236623e4614e4f8acb4a0c3282df58a289"
                )
            );
        }

        // Taken from: https://github.com/Consensys/gnark-crypto/blob/21087f253efe4681b43c0ff28b963459f6262b54/ecc/bn254/hash_vectors_test.go#L29
        #[test]
        fn gnark_compatibility_bn254_g1_hash_to_point() {
            const DST: &[u8] = b"QUUX-V01-CS02-with-BN254G1_XMD:SHA-256_SVDW_RO_";

            let msg = b"";
            let p = ark_bn254::Bn254::hash_to_g1_custom::<sha2::Sha256>(msg, DST).into_affine();
            assert_eq!(
                p,
                hex64_to_g1_bn254(
                    "0a976ab906170db1f9638d376514dbf8c42aef256a54bbd48521f20749e59e86",
                    "02925ead66b9e68bfc309b014398640ab55f6619ab59bc1fab2210ad4c4d53d5"
                )
            );

            let msg = b"abc";
            let p = ark_bn254::Bn254::hash_to_g1_custom::<sha2::Sha256>(msg, DST).into_affine();
            assert_eq!(
                p,
                hex64_to_g1_bn254(
                    "23f717bee89b1003957139f193e6be7da1df5f1374b26a4643b0378b5baf53d1",
                    "04142f826b71ee574452dbc47e05bc3e1a647478403a7ba38b7b93948f4e151d"
                )
            );

            let msg = b"abcdef0123456789";
            let p = ark_bn254::Bn254::hash_to_g1_custom::<sha2::Sha256>(msg, DST).into_affine();
            assert_eq!(
                p,
                hex64_to_g1_bn254(
                    "187dbf1c3c89aceceef254d6548d7163fdfa43084145f92c4c91c85c21442d4a",
                    "0abd99d5b0000910b56058f9cc3b0ab0a22d47cf27615f588924fac1e5c63b4d"
                )
            );
        }

        // Taken from: https://github.com/Consensys/gnark-crypto/blob/21087f253efe4681b43c0ff28b963459f6262b54/ecc/bn254/hash_vectors_test.go#L84
        #[test]
        fn gnark_compatibility_bn254_g2_hash_to_point() {
            const DST: &[u8] = b"QUUX-V01-CS02-with-BN254G2_XMD:SHA-256_SVDW_RO_";

            let msg = b"";
            let p = ark_bn254::Bn254::hash_to_g2_custom::<sha2::Sha256>(msg, DST).into_affine();
            assert_eq!(p, hex64_to_g2_bn254("1192005a0f121921a6d5629946199e4b27ff8ee4d6dd4f9581dc550ade8513001747d950a6f23c16156e2171bce95d1189b04148ad12628869ed21c96a8c9335", "0498f6bb5ac309a07d9a8b88e6ff4b8de0d5f27a075830e1eb0e68ea318201d82c9755350ca363ef2cf541005437221c5740086c2e909b71d075152484e845f4"));

            let msg = b"abc";
            let p: ark_ec::short_weierstrass::Projective<ark_bn254::g2::Config> =
                ark_bn254::Bn254::hash_to_g2_custom::<sha2::Sha256>(msg, DST);
            assert_eq!(p, hex64_to_g2_bn254("16c88b54eec9af86a41569608cd0f60aab43464e52ce7e6e298bf584b94fccd20b5db3ca7e8ef5edf3a33dfc3242357fbccead98099c3eb564b3d9d13cba4efd", "1c42ba524cb74db8e2c680449746c028f7bea923f245e69f89256af2d6c5f3ac22d02d2da7f288545ff8789e789902245ab08c6b1d253561eec789ec2c1bd630"));

            let msg = b"abcdef0123456789";
            let p: ark_ec::short_weierstrass::Projective<ark_bn254::g2::Config> =
                ark_bn254::Bn254::hash_to_g2_custom::<sha2::Sha256>(msg, DST);
            assert_eq!(p, hex64_to_g2_bn254("1435fd84aa43c699230e371f6fea3545ce7e053cbbb06a320296a2b81efddc702a8a360585b6b05996ef69c3c09b2c6fb17afe2b1e944f07559c53178eabf171", "2820188dcdc13ffdca31694942418afa1d6dfaaf259d012fab4da52b0f592e38142f08e2441ec431defc24621b73cfe0252d19b243cb55b84bdeb85de039207a"));
        }
    }
    #[cfg(feature = "bls12-381")]
    mod bls12_381 {
        use crate::hash_to_curve::CustomPairingHashToCurve;
        use ark_bls12_381::Bls12_381;
        use ark_ff::{BigInteger, PrimeField};
        use rstest::*;

        // RFC 9380 § J.9.1 test vectors
        #[rstest]
        #[case(
            b"",
            "052926add2207b76ca4fa57a8734416c8dc95e24501772c814278700eed6d1e4e8cf62d9c09db0fac349612b759e79a1",
            "08ba738453bfed09cb546dbb0783dbb3a5f1f566ed67bb6be0e8c67e2e81a4cc68ee29813bb7994998f3eae0c9c6a265",
        )]
        #[case(
            b"abc",
            "03567bc5ef9c690c2ab2ecdf6a96ef1c139cc0b2f284dca0a9a7943388a49a3aee664ba5379a7655d3c68900be2f6903",
            "0b9c15f3fe6e5cf4211f346271d7b01c8f3b28be689c8429c85b67af215533311f0b8dfaaa154fa6b88176c229f2885d",
        )]
        #[case(
            b"abcdef0123456789",
            "11e0b079dea29a68f0383ee94fed1b940995272407e3bb916bbf268c263ddd57a6a27200a784cbc248e84f357ce82d98",
            "03a87ae2caf14e8ee52e51fa2ed8eefe80f02457004ba4d486d6aa1f517c0889501dc7413753f9599b099ebcbbd2d709",
        )]
        #[case(
            b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
            "15f68eaa693b95ccb85215dc65fa81038d69629f70aeee0d0f677cf22285e7bf58d7cb86eefe8f2e9bc3f8cb84fac488",
            "1807a1d50c29f430b8cafc4f8638dfeeadf51211e1602a5f184443076715f91bb90a48ba1e370edce6ae1062f5e6dd38",
        )]
        #[case(
            b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "082aabae8b7dedb0e78aeb619ad3bfd9277a2f77ba7fad20ef6aabdc6c31d19ba5a6d12283553294c1825c4b3ca2dcfe",
            "05b84ae5a942248eea39e1d91030458c40153f3b654ab7872d779ad1e942856a20c438e8d99bc8abfbf74729ce1f7ac8",
        )]
        fn test_bls12381_g1_hash_to_point(
            #[case] msg: &[u8],
            #[case] p_x: &str,
            #[case] p_y: &str,
        ) {
            const DST: &[u8] = b"QUUX-V01-CS02-with-BLS12381G1_XMD:SHA-256_SSWU_RO_";

            let p = Bls12_381::hash_to_g1_custom::<sha2::Sha256>(msg, DST);
            assert_eq!(hex::encode(p.x.into_bigint().to_bytes_be()), p_x);
            assert_eq!(hex::encode(p.y.into_bigint().to_bytes_be()), p_y);
        }

        // RFC 9380 § J.10.1 test vectors
        #[rstest]
        #[case(
            b"",
            "0141ebfbdca40eb85b87142e130ab689c673cf60f1a3e98d69335266f30d9b8d4ac44c1038e9dcdd5393faf5c41fb78a",
            "05cb8437535e20ecffaef7752baddf98034139c38452458baeefab379ba13dff5bf5dd71b72418717047f5b0f37da03d",
            "0503921d7f6a12805e72940b963c0cf3471c7b2a524950ca195d11062ee75ec076daf2d4bc358c4b190c0c98064fdd92",
            "12424ac32561493f3fe3c260708a12b7c620e7be00099a974e259ddc7d1f6395c3c811cdd19f1e8dbf3e9ecfdcbab8d6",
        )]
        #[case(
            b"abc",
            "02c2d18e033b960562aae3cab37a27ce00d80ccd5ba4b7fe0e7a210245129dbec7780ccc7954725f4168aff2787776e6",
            "139cddbccdc5e91b9623efd38c49f81a6f83f175e80b06fc374de9eb4b41dfe4ca3a230ed250fbe3a2acf73a41177fd8",
            "1787327b68159716a37440985269cf584bcb1e621d3a7202be6ea05c4cfe244aeb197642555a0645fb87bf7466b2ba48",
            "00aa65dae3c8d732d10ecd2c50f8a1baf3001578f71c694e03866e9f3d49ac1e1ce70dd94a733534f106d4cec0eddd16",
            )]
        #[case(
            b"abcdef0123456789",
            "121982811d2491fde9ba7ed31ef9ca474f0e1501297f68c298e9f4c0028add35aea8bb83d53c08cfc007c1e005723cd0",
            "190d119345b94fbd15497bcba94ecf7db2cbfd1e1fe7da034d26cbba169fb3968288b3fafb265f9ebd380512a71c3f2c",
            "05571a0f8d3c08d094576981f4a3b8eda0a8e771fcdcc8ecceaf1356a6acf17574518acb506e435b639353c2e14827c8",
            "0bb5e7572275c567462d91807de765611490205a941a5a6af3b1691bfe596c31225d3aabdf15faff860cb4ef17c7c3be",
            )]
        #[case(
            b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
            "19a84dd7248a1066f737cc34502ee5555bd3c19f2ecdb3c7d9e24dc65d4e25e50d83f0f77105e955d78f4762d33c17da",
            "0934aba516a52d8ae479939a91998299c76d39cc0c035cd18813bec433f587e2d7a4fef038260eef0cef4d02aae3eb91",
            "14f81cd421617428bc3b9fe25afbb751d934a00493524bc4e065635b0555084dd54679df1536101b2c979c0152d09192",
            "09bcccfa036b4847c9950780733633f13619994394c23ff0b32fa6b795844f4a0673e20282d07bc69641cee04f5e5662",
            )]
        #[case(
            b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "01a6ba2f9a11fa5598b2d8ace0fbe0a0eacb65deceb476fbbcb64fd24557c2f4b18ecfc5663e54ae16a84f5ab7f62534",
            "11fca2ff525572795a801eed17eb12785887c7b63fb77a42be46ce4a34131d71f7a73e95fee3f812aea3de78b4d01569",
            "0b6798718c8aed24bc19cb27f866f1c9effcdbf92397ad6448b5c9db90d2b9da6cbabf48adc1adf59a1a28344e79d57e",
            "03a47f8e6d1763ba0cad63d6114c0accbef65707825a511b251a660a9b3994249ae4e63fac38b23da0c398689ee2ab52",
            )]
        fn test_bls12381_g2_hash_to_point(
            #[case] msg: &[u8],
            #[case] p_x0: &str,
            #[case] p_x1: &str,
            #[case] p_y0: &str,
            #[case] p_y1: &str,
        ) {
            const DST: &[u8] = b"QUUX-V01-CS02-with-BLS12381G2_XMD:SHA-256_SSWU_RO_";

            let p = Bls12_381::hash_to_g2_custom::<sha2::Sha256>(msg, DST);
            assert_eq!(hex::encode(p.x.c0.into_bigint().to_bytes_be()), p_x0);
            assert_eq!(hex::encode(p.x.c1.into_bigint().to_bytes_be()), p_x1);
            assert_eq!(hex::encode(p.y.c0.into_bigint().to_bytes_be()), p_y0);
            assert_eq!(hex::encode(p.y.c1.into_bigint().to_bytes_be()), p_y1);
        }
    }
}
