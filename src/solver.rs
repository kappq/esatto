use std::collections::HashMap;

use crate::formula::Formula;

pub enum SatResult {
    Sat(HashMap<u32, bool>),
    Unsat,
}

pub fn solve(formula: &Formula) -> SatResult {
    let mut assignment = HashMap::new();
    dpll(formula, &mut assignment)
}

fn dpll(formula: &Formula, assignment: &mut HashMap<u32, bool>) -> SatResult {
    // TODO: unit propagation

    match formula.eval(assignment) {
        Some(true) => SatResult::Sat(assignment.clone()),
        Some(false) => SatResult::Unsat,
        None => {
            if let Some(var) = choose_unassigned_var(formula, assignment) {
                assignment.insert(var, true);
                if let SatResult::Sat(assignment) = dpll(formula, assignment) {
                    return SatResult::Sat(assignment);
                }

                assignment.insert(var, false);
                if let SatResult::Sat(assignment) = dpll(formula, assignment) {
                    return SatResult::Sat(assignment);
                }

                assignment.remove(&var);
            }

            SatResult::Unsat
        }
    }
}

fn choose_unassigned_var(formula: &Formula, assignment: &HashMap<u32, bool>) -> Option<u32> {
    for clause in &formula.clauses {
        for lit in &clause.lits {
            let var = lit.var();
            if !assignment.contains_key(&var) {
                return Some(var);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{clause::Clause, lit::Lit};

    use super::*;

    #[test]
    fn test_sat() {
        let mut formula = Formula::new();

        let mut c1 = Clause::new();
        c1.add_literal(Lit::new(1, true));
        c1.add_literal(Lit::new(2, false));
        formula.add_clause(c1);

        let mut c2 = Clause::new();
        c2.add_literal(Lit::new(1, false));
        c2.add_literal(Lit::new(2, true));
        formula.add_clause(c2);

        assert!(matches!(solve(&formula), SatResult::Sat(_)));
    }

    #[test]
    fn test_unsat() {
        let mut formula = Formula::new();

        let mut c1 = Clause::new();
        c1.add_literal(Lit::new(1, true));
        formula.add_clause(c1);

        let mut c2 = Clause::new();
        c2.add_literal(Lit::new(1, false));
        formula.add_clause(c2);

        assert!(matches!(solve(&formula), SatResult::Unsat));
    }
}
