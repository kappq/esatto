mod clause;
mod formula;
mod lit;
mod parser;
mod var;

use crate::parser::parse;

fn main() {
    let formula = parse("test.dimacs");
    println!("{}", formula);
}
