use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum KzgError {
    /// The degree provided in setup was too small; degree 0 polynomials
    /// are not supported.
    #[error("degree 0 polynomial not supported")]
    #[non_exhaustive]
    DegreeIsZero,

    /// The degree of the polynomial passed to `commit` or `open`
    /// was too large.
    #[error("degree of the polynomial is too large `{num_coefficients}` > `{num_powers}`")]
    #[non_exhaustive]
    TooManyCoefficients {
        /// The number of coefficients in the polynomial.
        num_coefficients: usize,
        /// The maximum number of powers provided in `Powers`.
        num_powers: usize,
    },
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum KeyGenerationError {
    /// Error due to KZG.
    #[error(transparent)]
    Kzg(#[from] KzgError),

    /// Failed to create evaluation domain.
    #[error("failed to create evaluation domain")]
    #[non_exhaustive]
    EvaluationDomain,

    /// Failed to divide by vanishing polynomial
    #[error("failed to divide by vanishing polynomial")]
    #[non_exhaustive]
    VanishingPolyDiv,
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DecryptionError {
    /// Inconsistent number of partial decryptions / parties.
    #[error("partial_decryptions and parties must have the same length (got `{0}` != `{1}`)")]
    #[non_exhaustive]
    InconsistentPartiesArraysLength(usize, usize),

    /// Not enough partial decryptions / parties to decrypt ciphertext.
    #[error("not enough parties (got `{0}`, expected `{1}`)")]
    #[non_exhaustive]
    NotEnoughParties(usize, usize),

    /// Invalid CCA2 Proof.
    #[error("invalid cca2 proof")]
    #[non_exhaustive]
    InvalidCca2Proof,

    /// Failed verification.
    #[error(transparent)]
    Verification(#[from] VerificationError),

    /// Failed to create evaluation domain.
    #[error("failed to create evaluation domain")]
    #[non_exhaustive]
    EvaluationDomain,

    /// Error due to KZG.
    #[error(transparent)]
    Kzg(#[from] KzgError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum VerificationError {
    /// Invalid partial signature.
    #[error("invalid partial signature with index `{0}`")]
    #[non_exhaustive]
    InvalidPartialSig(usize),
}
