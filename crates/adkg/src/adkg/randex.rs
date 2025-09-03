use crate::adkg::randex::BerklekampWelchError::QDivE;
use crate::adkg::types::AdkgRandExMessage;
use crate::helpers::{PartyId, lagrange_interpolate_at, nth_powers, u64_from_usize};
use crate::vss::pedersen;
use ark_ec::CurveGroup;
use ark_ff::{Field, Zero};
use ark_poly::univariate::{DenseOrSparsePolynomial, DensePolynomial};
use ark_poly::{DenseUVPolynomial, Polynomial};
use nalgebra::{DMatrix, Dyn, OMatrix, RowDVector};

/// Attempts a round of OEC with the given messages.
pub fn oec_round<CG>(
    randex_messages: impl IntoIterator<Item = (PartyId, AdkgRandExMessage<CG::ScalarField>)>,
    t: usize,
    ped_commit: &CG,
    ped_g: &CG,
    ped_h: &CG,
) -> Option<(CG::ScalarField, CG::ScalarField)>
where
    CG: CurveGroup,
{
    let (points_z_i, points_z_hat_i): (Vec<_>, Vec<_>) = randex_messages
        .into_iter()
        .map(|(j, randex)| ((j.into(), randex.z_j), (j.into(), randex.z_hat_j)))
        .unzip();
    let msg_count = points_z_i.len();

    let e = {
        if msg_count == t + 1 {
            // We can either attempt to recover the shares with exactly t + 1 shares (assuming no errors),
            Some(0)
        } else if msg_count >= 2 * t + 2 {
            // Or do error correction with 2t + 1 + e shares (e > 0, number of allowed errors)
            Some(msg_count - 2 * t - 1)
        } else {
            tracing::debug!(
                msg_count,
                "Not enough messages to attempt online-error correction round"
            );
            None?
        }
    }?;

    let (z_i, z_hat_i) = if e == 0 {
        // No errors are tolerated, do a lagrange interpolation
        let z_i = lagrange_interpolate_at::<CG>(&points_z_i, 0);
        let z_hat_i = lagrange_interpolate_at::<CG>(&points_z_hat_i, 0);
        Some((z_i, z_hat_i))
    } else {
        tracing::info!(
            max_errors = e,
            msg_count,
            "Attempting share recovery with error correction"
        );

        // Do error correction on points_z_i and points_z_hat_i
        let rs_decode = |points: &[(u64, CG::ScalarField)]| {
            let poly_z = match berlekamp_welch(points, t, e) {
                Ok(poly_z) => poly_z,
                Err(err) => {
                    // Failed to decode polynomial, likely more than e errors.
                    tracing::error!(
                        error = ?err,
                        points_len = points.len(),
                        t,
                        min_errors = e,
                        "Failed to decode polynomial, likely due to too many errors"
                    );
                    None?
                }
            };

            // Get our share by evaluating the corrected polynomial at 0
            Some(poly_z.evaluate(&0u64.into()))
        };

        let z_i = rs_decode(&points_z_i)?;
        let z_hat_i = rs_decode(&points_z_hat_i)?;
        Some((z_i, z_hat_i))
    }?;

    // Verify the validity of z_i, z_hat_i by trying to open the expected commitment to (z_i, z_hat_i_points).
    // We verify correctness through the pedersen commitment, instead of verifying that less than 2t + 1 fragments
    // were corrupted.
    if pedersen::commit::open(&z_i, &z_hat_i, ped_commit, ped_g, ped_h) {
        Some((z_i, z_hat_i))
    } else {
        // Got at least min_errors > max_errors
        tracing::warn!(
            min_errors = e,
            "Got randex messages with too many invalid shares"
        );
        None
    }
}

/// During the randomness extraction phase, we use a Vandermonde matrix to obtain a secret-shared polynomial
/// $z(x) = z_0 + z_1 x + ... + z_\ell x^\ell$ of degree \ell (i.e., the reconstruction threshold).
///
/// Using a Vandermonde matrix and the ACSS secret shares $(a_i, b_i) i \in [n]$, we obtain the coefficients
/// of the polynomial as:
/// - $p(j)         = z_{j - 1}     = a_1 + j a_2 + ... + j^{n-1} a_n, j \in [1, t + 1]$
/// - $\tilde{p}(j) = z_{j + t}     = b_1 + j b_2 + ... + j^{n-1} b_n, j \in [1, \ell - t]$
///
/// Then, we need to evaluate the polynomial at $n$ points. This gives us;
/// $$
///     z(x) = (a_1 + ... + a_n)    + ... + (a_1 +    (t + 1) a_2 + ... +    (t + 1)^{n-1} a_n)x^{t - 1} +
///            (b_1 + ... + b_n)x^t + ... + (b_1 + (\ell - t) b_2 + ... + (\ell - t)^{n-1} b_n)x^\ell
///          = p(1) + p(2)x ... + p(t + 1)x^t + \tilde{p}(1)x^{t + 1} + ... + \tilde{p}(\ell - t)x^\ell
/// $$
///
/// In matrix form, we have
/// - $(p(1), ..., p(t + 1)) = V_{(t + 1) \times n} (a_1, ..., a_n)$,
/// - $(\tilde{p}(1), ..., \tilde{p}(\ell - t)) = V_{(\ell - t) \times n} (b_1, ..., b_n)$,
/// - $$
///   (z(1), ..., z(n)) = V_{n \times (t + 1)} (p(1), ..., p(t+1)) + V^{(t + 1)}_{n \times (\ell - t)} (\tilde{p}(1), ..., \tilde{p}(\ell - t)
///   = V_{n \times (t + 1)} V_{(t + 1) \times n} (a_1, ..., a_n) + V^{(t + 1)}_{n \times (\ell - t)} * V_{(\ell - t) \times n} (b_1, ..., b_n)
///   $$
///   where V is a Vandermonde matrix where the first row begins with 1, and V^{(t + 1)} starts with t + 1.
///
/// This function returns the matrices $V_{n \times (t + 1)} V_{(t + 1) \times n}$, and $V^{(t + 1)}_{n \times (\ell - t)} * V_{(\ell - t) \times n}$.
pub fn build_randex_matrices<T>(
    t: usize,
    ell: usize,
    n: usize,
) -> (OMatrix<T, Dyn, Dyn>, OMatrix<T, Dyn, Dyn>)
where
    T: Field,
{
    let make_vandermonde_flat_matrix = |rows: usize, cols: usize, exp_offset: u64| {
        let flat_m: Vec<T> = (1..=u64_from_usize(rows))
            .flat_map(|base| {
                let base = T::from(base);
                let mut m = Vec::with_capacity(cols);
                // Start with the initial power: base**exp_offset
                let mut val = base.pow([exp_offset]);
                for _ in 0..cols {
                    m.push(val);
                    // Multiply to get the next power for the next iteration
                    val *= base;
                }
                m
            })
            .collect();

        DMatrix::from_row_iterator(rows, cols, flat_m)
    };

    let v_n_t_p1 = make_vandermonde_flat_matrix(n, t + 1, 0);
    let v_t_p1_n = make_vandermonde_flat_matrix(t + 1, n, 0);

    let v_n_ell_mt_offset = make_vandermonde_flat_matrix(n, ell - t, t as u64 + 1);
    let v_ell_mt_n = make_vandermonde_flat_matrix(ell - t, n, 0);

    // Perform the matrix multiplications
    let m = v_n_t_p1 * &v_t_p1_n;
    let m_tilde = v_n_ell_mt_offset * &v_ell_mt_n;

    (m, m_tilde)
}

/// Solve the linear system A_{m x n} x_{n} = b_{m} using Gauss-Jordan elimination
/// Returns Some(x) if the system has a unique solution, None otherwise
pub fn gauss_jordan<T>(mut a: DMatrix<T>, mut b: RowDVector<T>) -> Option<RowDVector<T>>
where
    T: Field,
{
    let n = a.nrows();
    let m = a.ncols();

    if b.len() != n {
        panic!(
            "Dimension mismatch: A is {}x{}, b has length {}",
            n,
            m,
            b.len()
        );
    }

    let mut row = 0usize;
    let mut pivot_cols: Vec<usize> = Vec::new();

    for col in 0..m {
        if row >= n {
            break; // ran out of rows
        }

        // Find pivot in this column at or below `row`
        let Some(pivot_row) = (row..n).find(|&r| !a[(r, col)].is_zero()) else {
            continue; // no pivot in this column
        };

        // Swap in the pivot row
        if pivot_row != row {
            a.swap_rows(row, pivot_row);
            b.swap_rows(row, pivot_row);
        }

        // Normalize pivot row so that a[row, col] = 1
        let pivot = a[(row, col)];
        let inv_pivot = T::one() / pivot;
        for j in col..m {
            a[(row, j)] *= inv_pivot;
        }
        b[row] *= inv_pivot;

        for r in 0..n {
            if r != row {
                let coeff = a[(r, col)];
                if !coeff.is_zero() {
                    let factor = -coeff;

                    // Extract row r as a vector and use axpy
                    let mut row_r = a.row(r).transpose();
                    let row_pivot = a.row(row).transpose();
                    row_r.axpy(factor, &row_pivot, T::one());

                    // Copy the result back to the matrix
                    for j in 0..m {
                        a[(r, j)] = row_r[j];
                    }

                    // Apply same to b
                    b[r] = b[r] + factor * b[row];
                }
            }
        }

        pivot_cols.push(col);
        row += 1;
    }

    // Inconsistency check: [0 ... 0 | nonzero]
    for i in 0..n {
        if (0..m).all(|j| a[(i, j)].is_zero()) && !b[i].is_zero() {
            // no solution
            None?
        }
    }

    // Build a particular solution: free variables = 0
    let mut x = RowDVector::zeros(m);
    for (r, &c) in pivot_cols.iter().enumerate() {
        // After Gauss-Jordan, row r is e_c in columns, so x[c] = b[r]
        x[c] = b[r];
    }

    Some(x)
}

#[derive(thiserror::Error, Debug)]
pub enum BerklekampWelchError {
    #[error("failed to solve linear system")]
    NoSolution,

    #[error("failed to divide Q(x)/E(x)")]
    QDivE,
}

/// Implementation of the [Berlekamp-Welch algorithm](https://en.wikipedia.org/wiki/Berlekamp%E2%80%93Welch_algorithm)
/// to recover a polynomial of degree `t`, and correct up to `max_errors`.
pub fn berlekamp_welch<F>(
    points: &[(u64, F)],
    t: usize,
    max_errors: usize,
) -> Result<DensePolynomial<F>, BerklekampWelchError>
where
    F: Field,
{
    if points.len() < 2 * t + 1 {
        panic!("points.len() must be >= 2 * t + max_errors + 1")
    }

    let n = points.len();
    let e = max_errors;
    let deg_e = e;
    let deg_q = t + e;
    let num_vars = deg_q + deg_e + 1; // t + 2e + 1

    let mut m = DMatrix::<F>::zeros(n, num_vars);
    let mut b = Vec::with_capacity(n);

    // Build a system Mx = b
    for (i, &(x_i, y_i)) in points.iter().enumerate() {
        // Compute the num_vars-th powers of x_i, i.e., (1, x_i^1, ..., x_i^{deg_q})
        let x_i_pows: Vec<F> = nth_powers(&x_i.into(), deg_q + 1);
        let x_i_pow_e = x_i_pows[deg_e];

        let m_i_left = x_i_pows[0..deg_q + 1].to_vec();

        // Compute (-y_i, -y_i x^1, ..., -y_i^e)
        let m_i_right = x_i_pows[0..=e].iter().map(|x_i_pow_i| -y_i * x_i_pow_i);

        // Row i of M = [ 1 x_i ... x_i^{deg_q} | -y_i -y_i x_i ... -y_i x_i^{e} ] for i \in points
        let row = RowDVector::from_row_iterator(num_vars, m_i_left.into_iter().chain(m_i_right));
        m.set_row(i, &row);

        // Add y_i * x_i^e to vec
        b.push(y_i * x_i_pow_e)
    }

    let b = RowDVector::from_row_iterator(n, b);
    let x = gauss_jordan(m, b).ok_or(BerklekampWelchError::NoSolution)?;
    let coeffs: Vec<_> = x
        .into_iter()
        .copied()
        .chain(std::iter::once(F::one()))
        .collect(); // append 1 for the leading coeff of E

    let q_poly: DenseOrSparsePolynomial<_> =
        DensePolynomial::from_coefficients_slice(&coeffs[0..=deg_q]).into();
    let e_poly: DenseOrSparsePolynomial<_> =
        DensePolynomial::from_coefficients_slice(&coeffs[deg_q + 1..]).into();

    let (p_poly, remainder) = q_poly.divide_with_q_and_r(&e_poly).ok_or(QDivE)?;
    if !remainder.is_zero() {
        Err(QDivE)?
    }

    Ok(p_poly)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_poly::Polynomial;
    use nalgebra::DMatrix;

    #[test]
    fn test_generator_matrices() {
        let t = 1;
        let n = 3 * t + 1;
        let ell = 2 * t;
        let (m1, m2) = build_randex_matrices::<Fr>(t, ell, n);

        let m1: Vec<Fr> = m1
            .row_iter()
            .flat_map(|r| r.iter().cloned().collect::<Vec<_>>())
            .collect();
        let m2: Vec<Fr> = m2
            .row_iter()
            .flat_map(|r| r.iter().cloned().collect::<Vec<_>>())
            .collect();
        #[rustfmt::skip]
        assert_eq!(
            m1,
            vec![
                2.into(), 3.into(),  5.into(),  9.into(),
                3.into(), 5.into(),  9.into(), 17.into(),
                4.into(), 7.into(), 13.into(), 25.into(),
                5.into(), 9.into(), 17.into(), 33.into(),
            ],
        );
        #[rustfmt::skip]
        assert_eq!(
            m2,
            vec![
                 1.into(),  1.into(),  1.into(),  1.into(),
                 4.into(),  4.into(),  4.into(),  4.into(),
                 9.into(),  9.into(),  9.into(),  9.into(),
                16.into(), 16.into(), 16.into(), 16.into()
            ],
        );
    }

    // Helper function to create a polynomial from coefficients
    fn poly_from_coeffs(coeffs: &[Fr]) -> DensePolynomial<Fr> {
        DensePolynomial::from_coefficients_slice(coeffs)
    }

    // Helper function to evaluate polynomial at given points
    fn eval_poly_at_points(poly: &DensePolynomial<Fr>, points: &[u64]) -> Vec<(u64, Fr)> {
        points
            .iter()
            .map(|&x| (x, poly.evaluate(&Fr::from(x))))
            .collect()
    }

    // Helper function to introduce errors in evaluation points
    fn introduce_errors(mut points: Vec<(u64, Fr)>, error_positions: &[usize]) -> Vec<(u64, Fr)> {
        for &pos in error_positions {
            if pos < points.len() {
                // Change the y value to something random (but deterministic for testing)
                points[pos].1 += Fr::from(42u64);
            }
        }
        points
    }

    #[test]
    fn test_no_errors_degree_1() {
        // Test with a degree-1 polynomial: P(x) = 2x + 3
        let original_poly = poly_from_coeffs(&[Fr::from(3u64), Fr::from(2u64)]);
        let t = 1; // degree of original polynomial
        let max_errors = 1;
        let num_points = 2 * t + max_errors + 1; // = 4 points

        let points: Vec<u64> = (1..=num_points as u64).collect();
        let evaluations = eval_poly_at_points(&original_poly, &points);

        let result = berlekamp_welch(&evaluations, t, 1);
        let recovered_poly = result.unwrap();

        // Check that the recovered polynomial matches the original
        for &x in &points {
            let original_val = original_poly.evaluate(&Fr::from(x));
            let recovered_val = recovered_poly.evaluate(&Fr::from(x));
            assert_eq!(original_val, recovered_val, "Mismatch at x = {x}");
        }
    }

    #[test]
    fn test_no_errors_degree_2() {
        // Test with a degree-2 polynomial: P(x) = x^2 + 3x + 5
        let original_poly = poly_from_coeffs(&[Fr::from(5u64), Fr::from(3u64), Fr::from(1u64)]);
        let t = 2;
        let max_errors = 2;
        let num_points = 2 * t + max_errors + 1; // = 7 points

        let points: Vec<u64> = (1..=num_points as u64).collect(); // Use 1..=7 to avoid x=0 issues
        let evaluations = eval_poly_at_points(&original_poly, &points);

        let result = berlekamp_welch(&evaluations, t, max_errors);

        assert!(result.is_ok());
        let recovered_poly = result.unwrap();

        // Verify correctness
        for &x in &points {
            let original_val = original_poly.evaluate(&Fr::from(x));
            let recovered_val = recovered_poly.evaluate(&Fr::from(x));
            assert_eq!(original_val, recovered_val, "Mismatch at x = {x}");
        }
    }

    #[test]
    fn test_single_error_correction() {
        // Test error correction with 1 error
        let original_poly = poly_from_coeffs(&[Fr::from(1u64), Fr::from(2u64)]);
        let t = 1;
        let max_errors = 1;
        let num_points = 2 * t + max_errors + 1; // = 4 points

        let points: Vec<u64> = (1..=num_points as u64).collect();
        let clean_evaluations = eval_poly_at_points(&original_poly, &points);

        // Introduce 1 error at position 1
        let corrupted_evaluations = introduce_errors(clean_evaluations, &[1]);

        let result = berlekamp_welch(&corrupted_evaluations, t, max_errors);

        assert!(result.is_ok());
        let recovered_poly = result.unwrap();

        // Check that we recovered the original polynomial
        for &x in &points {
            let original_val = original_poly.evaluate(&Fr::from(x));
            let recovered_val = recovered_poly.evaluate(&Fr::from(x));
            assert_eq!(
                original_val, recovered_val,
                "Failed to correct error at x = {x}"
            );
        }
    }

    #[test]
    fn test_square_unique_solution_fr() {
        // System:
        // x + y = 3
        // x + 2y = 5
        // Solution over Fr: x=1, y=2
        let a = DMatrix::from_row_slice(
            2,
            2,
            &[
                Fr::from(1u64),
                Fr::from(1u64),
                Fr::from(1u64),
                Fr::from(2u64),
            ],
        );
        let b = RowDVector::from_row_slice(&[Fr::from(3u64), Fr::from(5u64)]);
        let sol = gauss_jordan(a, b).unwrap();
        assert_eq!(sol[0], Fr::from(1u64));
        assert_eq!(sol[1], Fr::from(2u64));
    }

    #[test]
    fn test_overdetermined_consistent_fr() {
        // Same system with a redundant equation:
        // x + y = 3
        // x + 2y = 5
        // 2x + 3y = 8  (redundant)
        let a = DMatrix::from_row_slice(
            3,
            2,
            &[
                Fr::from(1u64),
                Fr::from(1u64),
                Fr::from(1u64),
                Fr::from(2u64),
                Fr::from(2u64),
                Fr::from(3u64),
            ],
        );
        let b = RowDVector::from_row_slice(&[Fr::from(3u64), Fr::from(5u64), Fr::from(8u64)]);
        let sol = gauss_jordan(a, b).unwrap();
        assert_eq!(sol[0], Fr::from(1u64));
        assert_eq!(sol[1], Fr::from(2u64));
    }

    #[test]
    fn test_underdetermined_fr() {
        // x + y + z = 6
        // One equation, 3 variables. Particular solution with free vars = 0.
        let a = DMatrix::from_row_slice(1, 3, &[Fr::from(1u64), Fr::from(1u64), Fr::from(1u64)]);
        let b = RowDVector::from_row_slice(&[Fr::from(6u64)]);
        let sol = gauss_jordan(a, b).unwrap();
        assert_eq!(sol[0], Fr::from(6u64));
        assert_eq!(sol[1], Fr::from(0u64));
        assert_eq!(sol[2], Fr::from(0u64));
    }

    #[test]
    fn test_inconsistent_fr() {
        // x + y = 1
        // x + y = 2  (contradiction)
        let a = DMatrix::from_row_slice(
            2,
            2,
            &[
                Fr::from(1u64),
                Fr::from(1u64),
                Fr::from(1u64),
                Fr::from(1u64),
            ],
        );
        let b = RowDVector::from_row_slice(&[Fr::from(1u64), Fr::from(2u64)]);
        let out = gauss_jordan(a, b);
        assert!(out.is_none());
    }

    #[test]
    #[should_panic(expected = "Dimension mismatch")]
    fn test_bad_dims_fr() {
        let a = DMatrix::from_row_slice(
            2,
            2,
            &[
                Fr::from(1u64),
                Fr::from(0u64),
                Fr::from(0u64),
                Fr::from(1u64),
            ],
        );
        let b = RowDVector::from_row_slice(&[Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)]);
        let _ = gauss_jordan(a, b);
    }
}
