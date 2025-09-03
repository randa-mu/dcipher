use crate::helpers::u64_from_usize;
use ark_ff::Field;
use nalgebra::{DMatrix, Dyn, OMatrix};

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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;

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
}
