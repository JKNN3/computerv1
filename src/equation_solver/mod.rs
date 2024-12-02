mod equation_solver;
use equation_solver::*;
use std::collections::HashMap;

pub fn solver(mut terms: HashMap<i32, f64>) -> Result<(), String>{

    let degree = check_equation_degree(&mut terms)?;
    
    // let equation_terms = fill_struct(&terms);
    match degree{
        0 => println!("resolve degre 0"),
        1 => println!("resolve degre 1"),
        2 => println!("resolve degre 2"), 
        _ => println!("too big"),
    }
    Ok(())
}

    // check_equartion_degree
    // if ok fill the struct
    // check if there is an exponant > 2 if yy return err
    // else return struct of my terms