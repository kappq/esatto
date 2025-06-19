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
    use crate::clause::Clause;

    use super::*;

    #[test]
    fn test_sat() {
        let formula = Formula::from_clauses(vec![
            Clause::from_lits(vec![1, -2]),
            Clause::from_lits(vec![-1, 2]),
        ]);

        assert!(matches!(solve(&formula), SatResult::Sat(_)));
    }

    #[test]
    fn test_unsat() {
        let formula = Formula::from_clauses(vec![
            Clause::from_lits(vec![1]),
            Clause::from_lits(vec![-1]),
        ]);

        assert!(matches!(solve(&formula), SatResult::Unsat));
    }

    #[test]
    fn test_pigeonhole_unsat() {
        // 3 pigeons (A, B, C), 2 holes (X, Y)
        // 1 = pigeon A in hole X
        // 2 = pigeon A in hole Y
        // 3 = pigeon B in hole X
        // 4 = pigeon B in hole Y
        // 5 = pigeon C in hole X
        // 6 = pigeon C in hole Y

        let formula = Formula::from_clauses(vec![
            // Each pigeon in at least one hole
            Clause::from_lits(vec![1, 2]), // Pigeon A in hole X or in hole Y
            Clause::from_lits(vec![3, 4]), // Pigeon B in hole X or in hole Y
            Clause::from_lits(vec![5, 6]), // Pigeon C in hole X or in hole Y
            // No two pigeons in the same hole
            Clause::from_lits(vec![-1, -3]), // No pigeons A and B in hole X
            Clause::from_lits(vec![-3, -5]), // No pigeons B and C in hole X
            Clause::from_lits(vec![-1, -5]), // No pigeons A and C in hole X
            Clause::from_lits(vec![-2, -4]), // No pigeons A and B in hole Y
            Clause::from_lits(vec![-4, -6]), // No pigeons B and C in hole Y
            Clause::from_lits(vec![-2, -6]), // No pigeons A and C in hole Y
        ]);

        // For the pigeonhole principle, these conditions cannot be satisfied
        // and there must be at least one hole containing at least two pigeons
        assert!(matches!(solve(&formula), SatResult::Unsat));
    }

    #[test]
    fn test_graph_coloring_sat() {
        // Two colors (Red, Black)
        // Two nodes (A, B)
        // A and B are connected
        //
        // A - B
        //
        // 1 = A colored Red
        // 2 = A colored Black
        // 3 = B colored Red
        // 4 = B colored Black

        let formula = Formula::from_clauses(vec![
            // Each node must be colored Red or Black
            Clause::from_lits(vec![1, 2]),
            Clause::from_lits(vec![3, 4]),
            // No node colored can be colored both Red and Black
            Clause::from_lits(vec![-1, -2]),
            Clause::from_lits(vec![-3, -4]),
            // No connected nodes can be colored the same
            Clause::from_lits(vec![-1, -3]),
            Clause::from_lits(vec![-2, -4]),
        ]);

        assert!(matches!(solve(&formula), SatResult::Sat(_)));
    }

    #[test]
    fn test_graph_coloring_unsat() {
        // Two colors (Red, Black)
        // Three nodes (A, B, C)
        // All nodes are connected to each other
        //
        // A - B
        //  \ /
        //   C
        //
        // 1 = A colored Red
        // 2 = A colored Black
        // 3 = B colored Red
        // 4 = B colored Black
        // 5 = C colored Red
        // 6 = C colored Black

        let formula = Formula::from_clauses(vec![
            // Each node must be colored Red or Black
            Clause::from_lits(vec![1, 2]),
            Clause::from_lits(vec![3, 4]),
            Clause::from_lits(vec![5, 6]),
            // No node colored can be colored both Red and Black
            Clause::from_lits(vec![-1, -2]),
            Clause::from_lits(vec![-3, -4]),
            Clause::from_lits(vec![-5, -6]),
            // No connected nodes can be colored the same
            Clause::from_lits(vec![-1, -3]),
            Clause::from_lits(vec![-1, -5]),
            Clause::from_lits(vec![-3, -5]),
            Clause::from_lits(vec![-2, -4]),
            Clause::from_lits(vec![-2, -6]),
            Clause::from_lits(vec![-4, -6]),
        ]);

        assert!(matches!(solve(&formula), SatResult::Unsat));
    }
}
