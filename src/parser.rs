use std::str::FromStr;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0, space1},
    combinator::{map_res, opt, recognize},
    multi::{many0, separated_list0},
};

use crate::clause::Clause;
use crate::formula::Formula;
use crate::lit::Lit;

pub fn parse(filename: &str) -> Formula {
    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let (_, formula) = parse_dimacs(&input).expect("Failed to parse file");
    formula
}

fn parse_dimacs(input: &str) -> IResult<&str, Formula> {
    let (input, (_num_vars, _num_clauses)) = parse_header(input)?;
    let (input, formula) = parse_formula(input)?;

    Ok((input, formula))
}

fn parse_header(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = tag("p")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("cnf")(input)?;
    let (input, _) = space1(input)?;
    let (input, num_vars) = map_res(digit1, u32::from_str).parse(input)?;
    let (input, _) = space1(input)?;
    let (input, num_clauses) = map_res(digit1, u32::from_str).parse(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, (num_vars, num_clauses)))
}

fn parse_formula(input: &str) -> IResult<&str, Formula> {
    let (input, clauses) = many0(parse_clause).parse(input)?;

    let mut formula = Formula::new();
    for clause in clauses {
        formula.add_clause(clause);
    }

    Ok((input, formula))
}

fn parse_clause(input: &str) -> IResult<&str, Clause> {
    let (input, lits) = separated_list0(
        space1,
        map_res(recognize((opt(tag("-")), digit1)), i32::from_str),
    )
    .parse(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = line_ending(input)?;

    let mut clause = Clause::new();
    for lit in lits {
        if lit == 0 {
            break;
        }
        clause.add_literal(Lit::new(lit.abs() as u32, lit > 0));
    }

    Ok((input, clause))
}
