use esatto::{Clause, Formula, Lit, SatResult, solve};

fn encode(i: usize, j: usize, n: usize) -> u32 {
    ((i << 4) | (j << 2) | n) as u32
}

fn main() {
    // Let's solve the following 4x4 sudoku
    // +---+---+---+---+
    // | _ | _ | 3 | _ |
    // +---+---+---+---+
    // | _ | _ | 1 | _ |
    // +---+---+---+---+
    // | _ | _ | _ | 1 |
    // +---+---+---+---+
    // | 3 | _ | 2 | _ |
    // +---+---+---+---+
    //
    // The rules are as follows:
    // - Every row must contain every number
    // - Every column must contain every number
    // - No cell contains more than one number

    let mut formula = Formula::new();

    // Let p(i,j,n) denote that the cell in row i and column j has the value n
    // p(i,j,n) is going to be encoded as the variable (i<<4) | (j<<2) | n
    //
    // NOTE: positions and values range from 0 to 3 (inclusive) so that they fit into 2 bits

    // Add given digits as unit clauses
    formula.add_clause(Clause::from_lits(vec![Lit::new(encode(0, 2, 2), true)]));
    formula.add_clause(Clause::from_lits(vec![Lit::new(encode(1, 2, 0), true)]));
    formula.add_clause(Clause::from_lits(vec![Lit::new(encode(2, 3, 0), true)]));
    formula.add_clause(Clause::from_lits(vec![Lit::new(encode(3, 0, 2), true)]));
    formula.add_clause(Clause::from_lits(vec![Lit::new(encode(3, 2, 1), true)]));

    // Every row must contain every number
    // For every row and every number, one of the cells of the row must contain the number
    for i in 0..4 {
        for n in 0..4 {
            let mut clause = Clause::new();
            for j in 0..4 {
                let lit = Lit::new(encode(i, j, n), true);
                clause.add_literal(lit);
            }
            formula.add_clause(clause);
        }
    }

    // Every column must contain every number
    // For every column and every number, one of the cells of the column must contain the number
    for j in 0..4 {
        for n in 0..4 {
            let mut clause = Clause::new();
            for i in 0..4 {
                let lit = Lit::new(encode(i, j, n), true);
                clause.add_literal(lit);
            }
            formula.add_clause(clause);
        }
    }

    // No cell contains more than one number
    // For every cell, picked any two different values, that cell does not contain both of them
    for i in 0..4 {
        for j in 0..4 {
            for a in 0..4 {
                let lit_a = Lit::new(encode(i, j, a), false);
                for b in 0..4 {
                    if a == b {
                        continue;
                    }
                    let lit_b = Lit::new(encode(i, j, b), false);
                    formula.add_clause(Clause::from_lits(vec![lit_a, lit_b]));
                }
            }
        }
    }

    match solve(&formula) {
        SatResult::Sat(assignment) => {
            for i in 0..4 {
                println!("+---+---+---+---+");
                for j in 0..4 {
                    print!("| ");
                    for n in 0..4 {
                        if let Some(value) = assignment.get(&encode(i, j, n)) {
                            if *value {
                                print!("{} ", n + 1);
                                break;
                            }
                        }
                    }
                }
                println!("|");
            }
            println!("+---+---+---+---+");
        }
        SatResult::Unsat => {
            println!("The sudoku has no solution");
        }
    }
}
