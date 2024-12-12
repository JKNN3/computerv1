mod equation_solver;
mod parser;
use crate::equation_solver::solver;
use crate::parser::parser;

fn main() -> Result<(), String> {
    match parser() {
        Ok(equation_terms) => solver(equation_terms),
        Err(err) => Err(err),
    }
}
