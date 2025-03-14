use std::fmt;
use std::sync::Arc;

use crate::error::{Error, Result};
use crate::model::{Formula, Statement, Term};

/// Represents a single step in a proof
#[derive(Debug, Clone)]
pub struct ProofStep {
    /// The formula derived in this step
    pub conclusion: Formula,
    
    /// The rule or axiom used to make this step
    pub rule: String,
    
    /// The premises used in this step (references to previous steps)
    pub premises: Vec<usize>,
    
    /// Optional description of this step
    pub description: Option<String>,
}

impl ProofStep {
    /// Create a new proof step
    pub fn new(conclusion: Formula, rule: &str, premises: Vec<usize>) -> Self {
        ProofStep {
            conclusion,
            rule: rule.to_string(),
            premises,
            description: None,
        }
    }
    
    /// Add a description to this proof step
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

impl fmt::Display for ProofStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Step using rule '{}' with {} premise(s)", 
               self.rule, self.premises.len())?;
        
        if let Some(desc) = &self.description {
            write!(f, ": {}", desc)?;
        }
        
        Ok(())
    }
}

/// Represents a complete proof as a sequence of proof steps
#[derive(Debug, Clone)]
pub struct Proof {
    /// The steps in the proof
    pub steps: Vec<ProofStep>,
    
    /// The goal formula that the proof aims to establish
    pub goal: Option<Formula>,
}

impl Proof {
    /// Create a new empty proof
    pub fn new() -> Self {
        Proof {
            steps: Vec::new(),
            goal: None,
        }
    }
    
    /// Set the goal of the proof
    pub fn with_goal(mut self, goal: Formula) -> Self {
        self.goal = Some(goal);
        self
    }
    
    /// Add a step to the proof
    pub fn add_step(&mut self, step: ProofStep) -> usize {
        let step_index = self.steps.len();
        self.steps.push(step);
        step_index
    }
    
    /// Check if the proof is valid
    pub fn is_valid(&self) -> Result<bool> {
        // Check that all premise references in each step are valid
        for (i, step) in self.steps.iter().enumerate() {
            for &premise_idx in &step.premises {
                if premise_idx >= i {
                    return Err(Error::ProofVerificationError(
                        format!("Step {} references future step {}", i, premise_idx)
                    ));
                }
            }
        }
        
        // Check if the proof establishes its goal
        if let Some(goal) = &self.goal {
            if let Some(last_step) = self.steps.last() {
                // Compare the last step's conclusion with the goal
                // This is a simplistic check; a real implementation would do more sophisticated
                // semantic comparison or entailment checking
                if last_step.conclusion.statements.len() != goal.statements.len() {
                    return Ok(false);
                }
                
                // TODO: Implement proper formula entailment check
                // For now, just a basic equality check
                for (i, stmt) in last_step.conclusion.statements.iter().enumerate() {
                    if i >= goal.statements.len() || *stmt != goal.statements[i] {
                        return Ok(false);
                    }
                }
                
                Ok(true)
            } else {
                // Empty proof can't establish a non-trivial goal
                Ok(false)
            }
        } else {
            // No goal specified, so we can't determine if the proof is valid
            Ok(!self.steps.is_empty())
        }
    }
}

impl Default for Proof {
    fn default() -> Self {
        Self::new()
    }
} 