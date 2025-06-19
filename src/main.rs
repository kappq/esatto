mod clause;
mod formula;
mod lit;
mod parser;
mod solver;

use crate::{
    parser::parse_dimacs,
    solver::{SatResult, solve},
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let formula = parse_dimacs(&args[1]);
    match formula {
        Ok(formula) => {
            println!("{}", formula);
            match solve(&formula) {
                SatResult::Sat(assignment) => {
                    println!("SAT");

                    let mut vars = assignment.keys().collect::<Vec<_>>();
                    vars.sort();
                    for var in vars {
                        if let Some(value) = assignment.get(var) {
                            println!("x{}: {}", var, value);
                        }
                    }
                }
                SatResult::Unsat => {
                    println!("UNSAT");
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
