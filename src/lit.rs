use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lit(u32);

impl Lit {
    pub fn new(var: u32, sign: bool) -> Lit {
        Lit(var << 1 | sign as u32)
    }

    pub fn var(self) -> u32 {
        self.0 >> 1
    }

    pub fn sign(self) -> bool {
        self.0 & 1 == 1
    }

    pub fn eval(self, assignment: &HashMap<u32, bool>) -> Option<bool> {
        assignment.get(&self.var()).map(|value| *value == self.sign())
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var = self.var();
        if self.sign() {
            write!(f, "x{}", var)
        } else {
            write!(f, "¬x{}", var)
        }
    }
}
