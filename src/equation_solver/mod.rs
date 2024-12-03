
mod print_librairy;
mod equation_solver;
use equation_solver::*;
use std::collections::HashMap;
use print_librairy::Print; // pas besoin de use display car il fait partie de print, comme un objet un peu

pub fn solver(mut terms: HashMap<i32, f64>) -> Result<(), String>{

    let degree = check_equation_degree(&mut terms)?;
    let eq_terms = fill_equation_struct(&terms);

   Print::PolynomialDegree(degree).display();

    match degree{
        0 => resolve_degre_0(&eq_terms),
        1 => resolve_degre_1(&eq_terms),
        2 => resolve_degre_2(&eq_terms), 
        _ => Print::PolynomialDegreeError.display(),
    }
    Ok(())
}