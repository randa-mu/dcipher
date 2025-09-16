use ark_ec::{CurveGroup, PrimeGroup, VariableBaseMSM};
use ark_ff::{Field, PrimeField};
use ark_std::{One, Zero};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Mul};

/// This structure represents the identifier of a party from 1 to n.
/// When indexing with PartyId in slices, this structures accesses the
/// (party id - 1)th index.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct PartyId(pub usize);

struct PartyIter {
    next: Option<PartyId>,
    n: usize,
}

impl Iterator for PartyIter {
    type Item = PartyId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.next?;
        self.next = if id.0 >= self.n {
            None
        } else {
            Some(PartyId(id.0 + 1))
        };
        Some(id)
    }
}

impl PartyId {
    pub const fn const_from(i: u32) -> Self {
        Self(i as usize)
    }

    pub const fn iter_all(n: usize) -> impl Iterator<Item = PartyId> {
        PartyIter {
            next: Some(Self(1)),
            n,
        }
    }

    pub fn from_index(i: usize) -> Self {
        Self(i + 1)
    }

    pub fn as_index(&self) -> usize {
        if self.0 == 0 {
            panic!("Invalid PartyId, should be greater than zero");
        }

        self.0 - 1
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl Display for PartyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(usize);

impl SessionId {
    pub const fn const_from(i: u32) -> Self {
        Self(i as usize)
    }

    pub fn iter_all(n: usize) -> impl Iterator<Item = Self> {
        (1..=n).map(Self)
    }

    pub fn from_index(i: usize) -> Self {
        Self(i + 1)
    }

    pub fn as_index(&self) -> usize {
        if self.0 == 0 {
            panic!("Invalid SessionId, should be greater than zero");
        }

        self.0 - 1
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for SessionId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<SessionId> for usize {
    fn from(value: SessionId) -> Self {
        value.0
    }
}

impl PartialEq<PartyId> for SessionId {
    fn eq(&self, other: &PartyId) -> bool {
        self.0 == other.0
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl From<SessionId> for PartyId {
    fn from(value: SessionId) -> Self {
        PartyId(value.0)
    }
}

/// Allows to index arrays directly using PartyId.
/// Outputs an element at index (party id - 1).
macro_rules! gen_newtype_usize_index_for {
    ($ty:ty) => {
        impl<T> Index<$ty> for [T] {
            type Output = T;

            fn index(&self, idx: $ty) -> &Self::Output {
                &self[idx.0 - 1] // may panic
            }
        }

        impl<T> IndexMut<$ty> for [T] {
            fn index_mut(&mut self, idx: $ty) -> &mut Self::Output {
                &mut self[idx.0 - 1] // may panic
            }
        }

        impl<T> Index<$ty> for Vec<T> {
            type Output = T;

            fn index(&self, idx: $ty) -> &Self::Output {
                &self[idx.0 - 1] // may panic
            }
        }

        impl<T> IndexMut<$ty> for Vec<T> {
            fn index_mut(&mut self, idx: $ty) -> &mut Self::Output {
                &mut self[idx.0 - 1] // may panic
            }
        }
    };
}

gen_newtype_usize_index_for!(SessionId);
gen_newtype_usize_index_for!(&SessionId);
gen_newtype_usize_index_for!(PartyId);
gen_newtype_usize_index_for!(&PartyId);

/// Converts a usize into a PartyId.
/// While the value is not checked, one should never try to convert 0.
impl From<usize> for PartyId {
    fn from(value: usize) -> Self {
        PartyId(value)
    }
}

/// Converts a u32 into a PartyId.
/// While the value is not checked, one should never try to convert 0.
impl From<u32> for PartyId {
    fn from(value: u32) -> Self {
        PartyId(usize_from_u32(value))
    }
}

/// Converts a PartyId into a usize.
impl From<PartyId> for usize {
    fn from(val: PartyId) -> Self {
        val.0
    }
}

impl From<&PartyId> for usize {
    fn from(val: &PartyId) -> Self {
        val.0
    }
}

/// Converts a PartyId into a u64.
impl From<PartyId> for u64 {
    fn from(val: PartyId) -> Self {
        u64_from_usize(val.0)
    }
}

impl From<&PartyId> for u64 {
    fn from(val: &PartyId) -> Self {
        u64_from_usize(val.0)
    }
}

/// Converts a SessionId into a u64.
impl From<SessionId> for u64 {
    fn from(val: SessionId) -> Self {
        u64_from_usize(val.0)
    }
}

impl From<&SessionId> for u64 {
    fn from(val: &SessionId) -> Self {
        u64_from_usize(val.0)
    }
}

/// We use conditional compilation to ensure that this conversion function
/// only exists if the architecture of the system is 32 or 64 bits, i.e.,
/// the conversion is safe since u32 always fits within an usize.
#[allow(unused)]
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[inline(always)]
pub const fn usize_from_u32(x: u32) -> usize {
    x as usize
}

/// We use conditional compilation to ensure that this conversion function
/// only exists if the architecture of the system is 32 or 64 bits, i.e.,
/// the conversion is safe since usize always fits within an usize.
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[inline(always)]
pub const fn u64_from_usize(x: usize) -> u64 {
    x as u64
}

/// Evaluate the polynomial f(x) = coeffs[0] + coeffs[1]x + ...
#[allow(unused)]
pub fn eval_poly<S, C>(x: &S, coeffs: &[C]) -> C
where
    C: Copy,
    S: Copy,
    for<'a> C: Mul<&'a S, Output = C>,
    for<'a> C: Add<&'a C, Output = C>,
{
    // Start from the last coefficient
    let mut result = *coeffs.last().expect("cannot have zero polynomial");

    // Process coefficients in reverse order with Horner's method, skipping the first one
    for coeff in coeffs.iter().rev().skip(1) {
        result = result * x + coeff;
    }

    result
}

/// Evaluate a polynomial formed by EC points using a MSM.
#[allow(unused)]
pub fn eval_poly_msm<CG: CurveGroup>(public_poly: &[CG], x: &CG::ScalarField) -> CG {
    let nth_powers = nth_powers(x, public_poly.len());
    CG::msm(&CG::normalize_batch(public_poly), &nth_powers)
        .expect("nth_powers always has the same length as public_poly")
}

/// Compute the nth power of a scalar, i.e., (s^0, s^1, ..., s^{n-1}).
#[allow(unused)]
pub fn nth_powers<F: Field>(s: &F, n: usize) -> Vec<F> {
    let mut powers = vec![];
    let mut current_power = F::ONE;
    for _ in 0..n {
        powers.push(current_power);
        current_power *= s;
    }

    powers
}

/// Lagrange interpolation of the polynomial defined by its points, evaluated at point eval_x.
pub fn lagrange_interpolate_at<CG: CurveGroup>(
    points: &[(u64, CG::ScalarField)],
    eval_x: u64,
) -> CG::ScalarField
where
    CG::ScalarField: PrimeField,
{
    let mut result = CG::ScalarField::zero();
    let eval_point: CG::ScalarField = eval_x.into();
    for (i, (x_i, y_i)) in points.iter().enumerate() {
        let mut numerator = CG::ScalarField::one();
        let mut denominator = CG::ScalarField::one();
        let x_i: CG::ScalarField = (*x_i).into();

        // Lagrange basis polynomial at eval_point
        for (j, (x_j, _)) in points.iter().enumerate() {
            let x_j: CG::ScalarField = (*x_j).into();
            if i != j {
                numerator *= eval_point - x_j; // (x - x_j)
                denominator *= x_i - x_j; // (x_i - x_j)
            }
        }
        let inv_denominator = denominator.inverse().unwrap(); // denominator cannot be null + in prime field due to constraint

        // y_i * L_i(eval_x)
        let lagrange_basis = numerator * inv_denominator;
        result += *y_i * lagrange_basis;
    }
    result
}

/// Lagrange interpolation of the polynomial defined by its points, evaluated at point eval_x.
pub fn lagrange_points_interpolate_at<G>(points: &[(u64, G)], eval_x: u64) -> G
where
    G: VariableBaseMSM + PrimeGroup,
    G::ScalarField: PrimeField,
{
    let eval_point: G::ScalarField = eval_x.into();
    let scalars = points
        .iter()
        .enumerate()
        .map(|(i, (x_i, _))| -> Result<G::ScalarField, ()> {
            let mut numerator = G::ScalarField::one();
            let mut denominator = G::ScalarField::one();
            let x_i: G::ScalarField = (*x_i).into();

            // Lagrange basis polynomial at eval_point
            for (j, (x_j, _)) in points.iter().enumerate() {
                let x_j: G::ScalarField = (*x_j).into();
                if i != j {
                    numerator *= eval_point - x_j; // (x - x_j)
                    denominator *= x_i - x_j; // (x_i - x_j)
                }
            }

            // L_i(x_i)
            Ok(numerator * denominator.inverse().expect("den"))
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap(); // todo: fix unwrap

    let bases: Vec<G> = points.iter().map(|(_, y)| *y).collect();
    let bases = G::batch_convert_to_mul_base(&bases);

    G::msm(&bases, &scalars).expect("msm failed: bases and scalars have different lengths")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::{Fr, G1Affine};
    use ark_ec::AffineRepr;
    use ark_ff::MontFp;

    // polynomial f(x) = f_0 + f_1 * x + f_2 * x**2
    const F: [Fr; 3] = [
        MontFp!("9748014378023394463131025643809515256339738514302126881655597630578176012884"),
        MontFp!("7584258123282436594970491156697423918438183915429604535012204837391766094153"),
        MontFp!("17687066864945260253727634096948735901435915372770524487001060949946283757318"),
    ];

    // polynomial f(x) G_1
    const F_G1: [G1Affine; 3] = [
        G1Affine::new_unchecked(
            MontFp!(
                "12626450795289102133349107779181499611612252669247331498276177921181183086020"
            ),
            MontFp!("5259503020464854754742167077039084803028308602710496396880385385158964939643"),
        ),
        G1Affine::new_unchecked(
            MontFp!("5184210830636344962544211609922774538488382964516863091751241367597469213700"),
            MontFp!(
                "14808650705929504468802696135782185993840968799497957673480948521384237249462"
            ),
        ),
        G1Affine::new_unchecked(
            MontFp!("4824956371748783576392047927031975272813321586562115581617596910328958008462"),
            MontFp!("2769052689179680078498443316647748531371976532382731685677017463329961909032"),
        ),
    ];

    // f(i)
    const Y: [Fr; 4] = [
        F[0], // f(0) == f_0
        MontFp!("13131096494411816089582745152198399987665473402086221559970659231340417368738"),
        MontFp!("8111826597012207778996921363970206344766310234579296524891434358843609247994"),
        MontFp!("16578447557663844753619960024382209416190613412197386120116127199663560146269"),
    ];

    // f(i)*G
    const Y_G1: [G1Affine; 4] = [
        F_G1[0],
        G1Affine::new_unchecked(
            MontFp!("9351326506877260814914656560330755996772739388999949779258026219662786176357"),
            MontFp!("7762403044321814820649247167209054203680859027935444934787410678831935030834"),
        ),
        G1Affine::new_unchecked(
            MontFp!(
                "13016007432653321711867020727395508890293189744785136948453469103215721754039"
            ),
            MontFp!("4189332543690762371313428809302490563769951188887018556686272225429000215353"),
        ),
        G1Affine::new_unchecked(
            MontFp!(
                "12241683118745535806039192508220922276590973955419176484469658839964545520187"
            ),
            MontFp!(
                "20622166728909535500849187262346002847008294443838237880148251311753745549842"
            ),
        ),
    ];

    #[test]
    fn kat_lagrange_interpolate_at() {
        let points = Y
            .iter()
            .enumerate()
            .map(|(i, &y)| (i as u64, y))
            .collect::<Vec<_>>();

        let polynomial = (0..Y_G1.len())
            .map(|i| {
                lagrange_interpolate_at::<ark_bn254::G1Projective>(points.as_slice(), i as u64)
            })
            .collect::<Vec<_>>();
        assert_eq!(polynomial, Y);
    }

    #[test]
    fn kat_lagrange_points_interpolate_at() {
        let points = Y_G1
            .iter()
            .enumerate()
            .map(|(i, y)| (i as u64, y.into_group()))
            .collect::<Vec<_>>();

        let polynomial = (0..Y_G1.len())
            .map(|i| lagrange_points_interpolate_at(points.as_slice(), i as u64).into_affine())
            .collect::<Vec<_>>();
        assert_eq!(polynomial, Y_G1);
    }
}
