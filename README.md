# N3Proof

A Notation3 (N3) proof engine implemented in Rust.

## Overview

N3Proof is a library for working with [Notation3](https://www.w3.org/TeamSubmission/n3/) (N3), an assertion and logic language which is a superset of RDF. N3 extends the RDF data model by adding formulas (graphs), variables, logical implications, and functional predicates, allowing it to express rules and logic.

This library aims to provide:

1. Parsing and serialization of N3 documents
2. A representation of N3 logic formulas and rules
3. A proof engine for deriving new knowledge using N3 rules
4. Validation of N3 proofs

## Current Status

This project is in its early development stages. The current version provides:

- Basic data structures for representing N3 terms, statements, and formulas
- Groundwork for a proof engine architecture
- A simple proof validation mechanism

## Project Structure

The library is organized into several modules:

- **model**: Core data structures for N3 terms, statements, and formulas
- **parser**: Parsing N3 documents into the internal representation
- **reasoner**: The proof engine that applies rules to derive new knowledge
- **proof**: Representation and validation of proofs
- **error**: Error types for the library
- **utils**: Utility functions for working with N3 data

## Usage

```rust
use n3proof::{create_proof_engine, Formula, Statement, Term};

// Create a new proof engine
let mut engine = create_proof_engine();

// Add axioms and rules
// ...

// Perform reasoning
// ...

// Get and validate a proof
let proof = engine.get_proof();
match proof.is_valid() {
    Ok(true) => println!("Proof is valid!"),
    Ok(false) => println!("Proof is invalid."),
    Err(e) => println!("Error validating proof: {}", e),
}
```

## Features

- **N3 Data Model**: Representation of N3 terms, statements, formulas, and graphs
- **Proof Engine**: Forward-chaining reasoning for N3 rules
- **Proof Validation**: Verification of proof correctness

## Implementation Details

### N3 Terms

The library represents N3 terms with the following variants:

- IRIs (URIs)
- Blank nodes
- Literals (strings, numbers, etc.)
- Variables
- Nested formulas

### Formulas and Rules

Formulas are collections of statements with optional variable quantification. Rules are represented as premises and a conclusion, which can be applied to derive new knowledge.

### Proof Engine

The proof engine maintains a knowledge base and applies rules to derive new statements. Each step in the reasoning process is recorded in a proof, which can be validated for correctness.

## Roadmap

- [x] Basic data structures
- [x] Proof representation
- [ ] Complete N3 parser
- [ ] Proper unification algorithm
- [ ] Forward-chaining reasoner
- [ ] Backward-chaining reasoner
- [ ] N3 built-in functions support

## License

This project is licensed under the MIT License - see the LICENSE file for details. 