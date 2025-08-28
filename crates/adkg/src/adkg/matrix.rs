use crate::helpers::u64_from_usize;

// Randomness extraction phase
pub type Matrix<F> = Vec<Vec<F>>;

/// Multiplies two matrices
fn mat_mul(a: &Matrix<u64>, b: &Matrix<u64>) -> Matrix<u64> {
    let a_rows = a.len();
    let a_cols = a.first().map_or(0, |row| row.len());
    let b_rows = b.len();
    let b_cols = b.first().map_or(0, |row| row.len());

    // The number of columns in 'a' must equal the number of rows in 'b'.
    assert_eq!(
        a_cols, b_rows,
        "Incompatible matrix dimensions for multiplication"
    );

    let mut result = vec![vec![0u64; b_cols]; a_rows];

    for i in 0..a_rows {
        for j in 0..b_cols {
            result[i][j] = (0..a_cols).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    result
}

/// Build pre-processed Vandermonde that can be used to evaluate a polynomial at f(1), ..., f(n)
pub fn build_generator_matrices<T>(t: usize, deg: usize, n: usize) -> (Matrix<T>, Matrix<T>)
where
    T: From<u64>, // The output type T must be convertible from u64
{
    let make_matrix = |rows: usize, cols: usize, exp_offset: u32| -> Matrix<u64> {
        (1..=u64_from_usize(rows))
            .map(|base| {
                let mut row = Vec::with_capacity(cols);
                // Start with the initial power: base**exp_offset
                let mut val = base.pow(exp_offset);
                for _ in 0..cols {
                    row.push(val);
                    // Multiply to get the next power for the next iteration
                    val *= base;
                }
                row
            })
            .collect()
    };

    let high_deg_count = deg - t;
    let coeff_1 = make_matrix(n, t + 1, 0);
    let hm_1 = make_matrix(t + 1, n, 0);
    let coeff_2 = make_matrix(n, high_deg_count, t as u32 + 1);
    let hm_2 = make_matrix(high_deg_count, n, 0);

    // Perform the matrix multiplications
    let rm_1_u64 = mat_mul(&coeff_1, &hm_1);
    let rm_2_u64 = mat_mul(&coeff_2, &hm_2);

    // Convert the u64 matrices to the desired output type T
    let convert = |matrix_u64: Matrix<u64>| -> Matrix<T> {
        matrix_u64
            .into_iter()
            .map(|row| row.into_iter().map(T::from).collect())
            .collect()
    };

    (convert(rm_1_u64), convert(rm_2_u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_matrices() {
        let (m1, m2) = build_generator_matrices::<u64>(1, 2, 4);
        assert_eq!(
            m1,
            vec![[2, 3, 5, 9], [3, 5, 9, 17], [4, 7, 13, 25], [5, 9, 17, 33]]
        );
        assert_eq!(
            m2,
            vec![[1, 1, 1, 1], [4, 4, 4, 4], [9, 9, 9, 9], [16, 16, 16, 16]]
        );
    }
}
