
// struct term{
//     quadratic: <f64>,
//     linear: <f64>,
//     constant:<f64>,
// };


mod parser;
mod equation_solver;
use crate::parser::parser;
use crate::equation_solver::solver;

fn main() -> Result<(), String>{

    let equation_terms = parser()?;
    // dbg!(&equation_terms);
    solver(equation_terms)

// 
    // match parser(){
        // Ok(equation_terms) => {
            // solver(equation_terms),
        // }
        // Err(err) => Err(err),
    // }
}
