mod equation_solver;
use equation_solver::*;
use std::collections::HashMap;

pub fn solver(mut terms: HashMap<i32, f64>) -> Result<(), String>{

    let degree = check_equation_degree(&mut terms)?;
    let eq_terms = fill_equation_struct(&terms);

    // print reduced form
    
    println!("Polynomial degree: {}", degree);// print degree

    match degree{
        0 => resolve_degre_0(&eq_terms),
        1 => resolve_degre_1(&eq_terms),
        2 => resolve_degre_2(&eq_terms), 
        _ => println!("The polynomial degree is strictly greater than 2, I can't solve."),
    }
    Ok(())
}