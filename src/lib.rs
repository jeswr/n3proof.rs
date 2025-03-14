// n3proof - A Notation3 (N3) proof engine implemented in Rust
//
// This library implements a proof engine for Notation3 (N3), an extension of RDF
// that adds logical expressions and rules for the Semantic Web.

mod error;
mod model;
mod parser;
mod proof;
mod reasoner;
mod utils;

pub use error::Error;
pub use model::{Formula, Graph, Statement, Term};
pub use parser::{parse_n3, ParseOptions};
pub use proof::{Proof, ProofStep};
pub use reasoner::{ProofEngine, Rule};

/// The main entry point for creating a new proof engine
pub fn create_proof_engine() -> ProofEngine {
    ProofEngine::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_proof_test() {
        // This will be replaced with actual tests once we implement the core functionality
        let engine = create_proof_engine();
        assert!(engine.ready());
    }
}
