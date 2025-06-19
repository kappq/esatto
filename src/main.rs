mod clause;
mod formula;
mod lit;
mod parser;

use crate::parser::parse_dimacs;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let formula = parse_dimacs(&args[1]);
    match formula {
        Ok(formula) => println!("{}", formula),
        Err(err) => eprintln!("{}", err),
    }
}
