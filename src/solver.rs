use std::collections::HashMap;

use crate::{formula::Formula, lit::Lit};

pub enum SatResult {
    Sat(HashMap<u32, bool>),
    Unsat,
}

pub fn solve(formula: &Formula) -> SatResult {
    let assignment = HashMap::new();
    dpll(formula, assignment)
}

fn dpll(formula: &Formula, mut assignment: HashMap<u32, bool>) -> SatResult {
    while let Some(lit) = get_unit_lit(formula, &assignment) {
        assignment.insert(lit.var(), lit.sign());
    }

    match formula.eval(&assignment) {
        Some(true) => SatResult::Sat(assignment.clone()),
        Some(false) => SatResult::Unsat,
        None => {
            if let Some(var) = choose_unassigned_var(formula, &assignment) {
                let mut assignment_true = assignment.clone();
                assignment_true.insert(var, true);
                if let SatResult::Sat(assignment_true) = dpll(formula, assignment_true) {
                    return SatResult::Sat(assignment_true);
                }

                let mut assignment_false = assignment.clone();
                assignment_false.insert(var, false);
                if let SatResult::Sat(assignment_false) = dpll(formula, assignment_false) {
                    return SatResult::Sat(assignment_false);
                }
            }

            SatResult::Unsat
        }
    }
}

fn get_unit_lit(formula: &Formula, assignment: &HashMap<u32, bool>) -> Option<Lit> {
    for clause in &formula.clauses {
        if clause.eval(assignment) == Some(true) {
            continue;
        }
        let unassigned_lits = clause
            .lits
            .iter()
            .filter(|lit| !assignment.contains_key(&lit.var()))
            .collect::<Vec<_>>();
        if unassigned_lits.len() == 1 {
            return Some(*unassigned_lits[0]);
        }
    }
    None
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
