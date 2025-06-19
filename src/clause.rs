use std::collections::HashMap;

use crate::lit::Lit;

#[derive(Debug)]
pub struct Clause {
    pub lits: Vec<Lit>,
}

impl Clause {
    pub fn new() -> Self {
        Self { lits: Vec::new() }
    }

    pub fn add_literal(&mut self, lit: Lit) {
        self.lits.push(lit);
    }

    pub fn eval(&self, assignment: &HashMap<u32, bool>) -> Option<bool> {
        if self
            .lits
            .iter()
            .any(|lit| lit.eval(assignment) == Some(true))
        {
            return Some(true);
        }

        if self.lits.iter().any(|lit| lit.eval(assignment).is_none()) {
            return None;
        }

        Some(false)
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self
            .lits
            .iter()
            .map(|lit| lit.to_string())
            .collect::<Vec<_>>()
            .join(" ∨ ");
        write!(f, "({})", buf)
    }
}
