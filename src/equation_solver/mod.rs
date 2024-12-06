mod equation_solver;
mod print_enum;
use equation_solver::*;
use print_enum::Print;
use std::collections::HashMap;

/*  check the degree of the equation, and depending
on the result, print an error or call equation
solving functions */
pub fn solver(mut terms: HashMap<i32, f64>) -> Result<(), String> {
    let eq_terms = fill_equation_struct(&terms);
    Print::ReducedForm(&terms).display();
    let degree = check_equation_degree(&mut terms);
    Print::PolynomialDegree(degree).display();

    match degree {
        0 => resolve_degre_0(&eq_terms),
        1 => resolve_degre_1(&eq_terms),
        2 => resolve_degre_2(&eq_terms),
        _ => Print::PolynomialDegreeError.display(),
    }
    Ok(())
}
