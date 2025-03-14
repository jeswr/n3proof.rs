use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use oxiri::Iri;
use oxrdf::{BlankNode, Literal, NamedNode, Subject, Term as OxTerm};

use crate::error::{Error, Result};

/// Represents a term in N3 (can be IRI, blank node, literal, variable, or formula)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// An IRI reference
    Iri(NamedNode),
    
    /// A blank node
    BlankNode(BlankNode),
    
    /// A literal value (string, integer, etc.)
    Literal(Literal),
    
    /// A variable
    Variable(String),
    
    /// A nested formula/graph
    Formula(Arc<Formula>),
}

// Custom Hash implementation to handle Formula
impl std::hash::Hash for Term {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Term::Iri(iri) => iri.hash(state),
            Term::BlankNode(bn) => bn.hash(state),
            Term::Literal(lit) => lit.hash(state),
            Term::Variable(var) => var.hash(state),
            Term::Formula(formula) => {
                // Use the pointer for hashing, as Formula doesn't implement Hash
                Arc::as_ptr(formula).hash(state);
            }
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Iri(iri) => write!(f, "<{}>", iri.as_str()),
            Term::BlankNode(bn) => write!(f, "_:{}", bn.as_str()),
            Term::Literal(lit) => write!(f, "\"{}\"", lit.value()),
            Term::Variable(var) => write!(f, "?{}", var),
            Term::Formula(_) => write!(f, "{{...}}"),
        }
    }
}

/// Represents a statement (triple) in N3
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Statement {
    /// Subject of the statement
    pub subject: Term,
    
    /// Predicate of the statement
    pub predicate: Term,
    
    /// Object of the statement
    pub object: Term,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} .", self.subject, self.predicate, self.object)
    }
}

/// Represents a formula (a set of statements that can be nested)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formula {
    /// The statements contained in this formula
    pub statements: Vec<Statement>,
    
    /// Variables that are universally quantified in this formula
    pub universal_vars: HashSet<String>,
    
    /// Variables that are existentially quantified in this formula
    pub existential_vars: HashSet<String>,
}

impl Formula {
    /// Creates a new empty formula
    pub fn new() -> Self {
        Formula {
            statements: Vec::new(),
            universal_vars: HashSet::new(),
            existential_vars: HashSet::new(),
        }
    }
    
    /// Adds a statement to the formula
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
    
    /// Adds a universal variable to the formula
    pub fn add_universal_var(&mut self, var: &str) {
        self.universal_vars.insert(var.to_string());
    }
    
    /// Adds an existential variable to the formula
    pub fn add_existential_var(&mut self, var: &str) {
        self.existential_vars.insert(var.to_string());
    }
}

impl Default for Formula {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an RDF graph (a set of triples)
#[derive(Debug, Clone)]
pub struct Graph {
    /// The statements in the graph
    pub statements: Vec<Statement>,
}

impl Graph {
    /// Creates a new empty graph
    pub fn new() -> Self {
        Graph {
            statements: Vec::new(),
        }
    }
    
    /// Adds a statement to the graph
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
    
    /// Converts an OxRDF triple to an N3 statement
    pub fn convert_triple(&self, subject: &Subject, predicate: &NamedNode, object: &OxTerm) -> Result<Statement> {
        let n3_subject = match subject {
            Subject::NamedNode(nn) => Term::Iri(nn.clone()),
            Subject::BlankNode(bn) => Term::BlankNode(bn.clone()),
        };
        
        let n3_predicate = Term::Iri(predicate.clone());
        
        let n3_object = match object {
            OxTerm::NamedNode(nn) => Term::Iri(nn.clone()),
            OxTerm::BlankNode(bn) => Term::BlankNode(bn.clone()),
            OxTerm::Literal(lit) => Term::Literal(lit.clone()),
        };
        
        Ok(Statement {
            subject: n3_subject,
            predicate: n3_predicate,
            object: n3_object,
        })
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
} 