use crate::model::{Formula, Statement, Term};

/// Convert a Formula to a string representation in N3 syntax
pub fn formula_to_n3_string(formula: &Formula) -> String {
    let mut output = String::new();
    
    // Add prefixes if needed (in a complete implementation)
    output.push_str("# N3 Formula\n\n");
    
    // Add quantification declarations
    if !formula.universal_vars.is_empty() {
        output.push_str("@forAll ");
        for var in &formula.universal_vars {
            output.push_str(&format!("?{} ", var));
        }
        output.push_str(".\n");
    }
    
    if !formula.existential_vars.is_empty() {
        output.push_str("@forSome ");
        for var in &formula.existential_vars {
            output.push_str(&format!("?{} ", var));
        }
        output.push_str(".\n");
    }
    
    if !formula.universal_vars.is_empty() || !formula.existential_vars.is_empty() {
        output.push('\n');
    }
    
    // Add statements
    for statement in &formula.statements {
        output.push_str(&format!("{}\n", statement));
    }
    
    output
}

/// Create a simple RDF triple statement
pub fn create_triple(subject: Term, predicate: Term, object: Term) -> Statement {
    Statement { subject, predicate, object }
}

/// Helper function to check if two Formulas are equivalent
/// 
/// This is a simplistic implementation that just checks if they have the same statements.
/// A more complete implementation would check for semantic equivalence.
pub fn formulas_equivalent(a: &Formula, b: &Formula) -> bool {
    if a.statements.len() != b.statements.len() {
        return false;
    }
    
    // This is a very basic check - a real implementation would be more sophisticated
    // and would account for blank node identifiers, variable renaming, etc.
    for stmt_a in &a.statements {
        if !b.statements.contains(stmt_a) {
            return false;
        }
    }
    
    true
} 