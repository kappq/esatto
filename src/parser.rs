use std::str::FromStr;

use crate::{clause::Clause, formula::Formula, lit::Lit};

#[derive(Debug)]
pub enum ParseError {
    Io(std::io::Error),
    // TODO: add error location and erroring line/token
    InvalidLine,
    InvalidToken,
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::Io(value)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(_value: std::num::ParseIntError) -> Self {
        ParseError::InvalidToken
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(io_err) => write!(f, "IO error: {}", io_err),
            ParseError::InvalidLine => write!(f, "Invalid line"),
            ParseError::InvalidToken => write!(f, "Invalid token"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_dimacs(filename: &str) -> Result<Formula, ParseError> {
    let contents = std::fs::read_to_string(filename)?;

    let mut formula = Formula::new();
    for line in contents.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();

        match tokens.as_slice() {
            ["c", ..] => continue,
            ["p", "cnf", num_vars_str, num_clauses_str] => {
                let _num_vars = u32::from_str(num_vars_str)?;
                let _num_clauses = u32::from_str(num_clauses_str)?;
                // TODO: check the number of variables and clauses
            }
            [lits_str @ .., "0"] => {
                let mut clause = Clause::new();
                for lit_str in lits_str {
                    let lit = i32::from_str(lit_str)?;
                    if lit == 0 {
                        break;
                    }
                    clause.add_literal(Lit::new(lit.abs() as u32, lit > 0));
                }
                formula.add_clause(clause);
            }
            _ => return Err(ParseError::InvalidLine),
        }
    }

    Ok(formula)
}
