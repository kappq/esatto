use crate::clause::Clause;

#[derive(Debug)]
pub struct Formula {
    clauses: Vec<Clause>,
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
}

impl std::fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self
            .clauses
            .iter()
            .map(|clause| clause.to_string())
            .collect::<Vec<String>>()
            .join(" ∧ ");
        write!(f, "{}", buf)
    }
}
