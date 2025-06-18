use crate::lit::Lit;

#[derive(Debug)]
pub struct Clause {
    lits: Vec<Lit>,
}

impl Clause {
    pub fn new() -> Self {
        Self { lits: Vec::new() }
    }

    pub fn add_literal(&mut self, lit: Lit) {
        self.lits.push(lit);
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self
            .lits
            .iter()
            .map(|lit| lit.to_string())
            .collect::<Vec<String>>()
            .join(" ∨ ");
        write!(f, "({})", buf)
    }
}
