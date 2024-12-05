
mod parser;
mod equation_solver;
use crate::parser::parser;
use crate::equation_solver::solver;

/*      TODO
    - readme
    - tests unitaires ? 
    - tests programme
    - resolution solutions complexes pour le degre 2

*/

fn main() -> Result<(), String>{

    match parser(){
        Ok(equation_terms) => {
            solver(equation_terms)
        }
        Err(err) => Err(err),
    }
}
