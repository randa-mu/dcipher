use ark_ec::{PrimeGroup, VariableBaseMSM};
use ark_ff::{Field, PrimeField};
use ark_std::One;

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
        .map(|(i, (x_i, _))| {
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
            numerator
                * denominator
                    .inverse() // we are in a prime field, i.e., all points have an inverse but zero
                    .expect("received two points with the same identifiers")
        })
        .collect::<Vec<_>>();

    let bases: Vec<G> = points.iter().copied().map(|(_, b)| b).collect();
    let bases = G::batch_convert_to_mul_base(&bases);
    G::msm(&bases, &scalars).expect("msm failed: bases and scalars have different lengths")
}
