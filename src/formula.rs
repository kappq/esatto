use std::collections::HashMap;

use crate::clause::Clause;

#[derive(Debug)]
pub struct Formula {
    pub clauses: Vec<Clause>,
}

impl Formula {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
        }
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }

    pub fn eval(&self, assignment: &HashMap<u32, bool>) -> Option<bool> {
        if self
            .clauses
            .iter()
            .any(|clause| clause.eval(assignment) == Some(false))
        {
            return Some(false);
        }

        if self
            .clauses
            .iter()
            .any(|clause| clause.eval(assignment).is_none())
        {
            return None;
        }

        Some(true)
    }
}

impl std::fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self
            .clauses
            .iter()
            .map(|clause| clause.to_string())
            .collect::<Vec<_>>()
            .join(" ∧ ");
        write!(f, "{}", buf)
    }
}
