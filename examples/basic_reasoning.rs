use oxrdf::{NamedNode, Literal};
use n3proof::{
    create_proof_engine,
    Formula, Statement, Term, Rule,
};

fn main() {
    // Create a new proof engine
    let mut engine = create_proof_engine();
    println!("Created a new N3 proof engine");
    
    // Create some example terms
    let person = Term::Iri(NamedNode::new("http://example.org/ns#Person").unwrap());
    let alice = Term::Iri(NamedNode::new("http://example.org/ns#Alice").unwrap());
    let bob = Term::Iri(NamedNode::new("http://example.org/ns#Bob").unwrap());
    let knows = Term::Iri(NamedNode::new("http://example.org/ns#knows").unwrap());
    let name = Term::Iri(NamedNode::new("http://example.org/ns#name").unwrap());
    let alice_name = Term::Literal(Literal::new_simple_literal("Alice"));
    let bob_name = Term::Literal(Literal::new_simple_literal("Bob"));
    let type_pred = Term::Iri(NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap());
    
    // Create some statements
    let alice_is_person = Statement {
        subject: alice.clone(),
        predicate: type_pred.clone(),
        object: person.clone(),
    };
    
    let bob_is_person = Statement {
        subject: bob.clone(),
        predicate: type_pred.clone(),
        object: person.clone(),
    };
    
    let alice_knows_bob = Statement {
        subject: alice.clone(),
        predicate: knows.clone(),
        object: bob.clone(),
    };
    
    let alice_has_name = Statement {
        subject: alice.clone(),
        predicate: name.clone(),
        object: alice_name.clone(),
    };
    
    let bob_has_name = Statement {
        subject: bob.clone(),
        predicate: name.clone(),
        object: bob_name.clone(),
    };
    
    // Create formulas from the statements
    let mut alice_formula = Formula::new();
    alice_formula.add_statement(alice_is_person);
    alice_formula.add_statement(alice_has_name);
    
    let mut bob_formula = Formula::new();
    bob_formula.add_statement(bob_is_person);
    bob_formula.add_statement(bob_has_name);
    
    let mut knows_formula = Formula::new();
    knows_formula.add_statement(alice_knows_bob);
    
    // Add the formulas as axioms to the engine
    let alice_idx = engine.add_axiom(alice_formula);
    let bob_idx = engine.add_axiom(bob_formula);
    let knows_idx = engine.add_axiom(knows_formula);
    
    println!("Added axioms to the proof engine:");
    println!("  Axiom {}: Alice is a Person and has name 'Alice'", alice_idx);
    println!("  Axiom {}: Bob is a Person and has name 'Bob'", bob_idx);
    println!("  Axiom {}: Alice knows Bob", knows_idx);
    
    // Create a simple rule: If X knows Y, then Y knows X (symmetry of knows)
    let mut premise_formula = Formula::new();
    let x_var = Term::Variable("X".to_string());
    let y_var = Term::Variable("Y".to_string());
    
    let x_knows_y = Statement {
        subject: x_var.clone(),
        predicate: knows.clone(),
        object: y_var.clone(),
    };
    
    premise_formula.add_statement(x_knows_y);
    premise_formula.add_universal_var("X");
    premise_formula.add_universal_var("Y");
    
    let mut conclusion_formula = Formula::new();
    let y_knows_x = Statement {
        subject: y_var.clone(),
        predicate: knows.clone(),
        object: x_var.clone(),
    };
    
    conclusion_formula.add_statement(y_knows_x);
    conclusion_formula.add_universal_var("X");
    conclusion_formula.add_universal_var("Y");
    
    let knows_symmetry_rule = Rule::new(
        "knows_symmetry",
        vec![premise_formula],
        conclusion_formula
    ).with_description("If X knows Y, then Y knows X");
    
    // Add the rule to the engine
    let rule_idx = engine.add_rule(knows_symmetry_rule);
    println!("Added rule: If X knows Y, then Y knows X");
    
    // Apply the rule
    match engine.apply_rule(rule_idx, &[knows_idx]) {
        Ok(result_idx) => {
            println!("Successfully applied rule and derived new knowledge:");
            println!("  Result {}: Bob knows Alice", result_idx);
        },
        Err(e) => {
            println!("Failed to apply rule: {}", e);
        }
    }
    
    // Get the proof
    let proof = engine.get_proof();
    println!("\nProof has {} steps:", proof.steps.len());
    
    for (i, step) in proof.steps.iter().enumerate() {
        println!("  Step {}: {}", i, step);
    }
} 