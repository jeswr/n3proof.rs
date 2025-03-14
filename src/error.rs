use thiserror::Error;

/// Represents errors that can occur in the N3 proof engine
#[derive(Error, Debug)]
pub enum Error {
    /// Error during parsing of N3 syntax
    #[error("Parsing error: {0}")]
    ParseError(String),

    /// Error during reasoning or proof generation
    #[error("Reasoning error: {0}")]
    ReasoningError(String),

    /// Error related to invalid term or formula
    #[error("Model error: {0}")]
    ModelError(String),

    /// Error when trying to verify a proof
    #[error("Proof verification error: {0}")]
    ProofVerificationError(String),

    /// Error forwarded from RDF libraries
    #[error("RDF error: {0}")]
    RdfError(#[from] anyhow::Error),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Shorthand for Result with our Error type
pub type Result<T> = std::result::Result<T, Error>; 