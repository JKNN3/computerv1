
use std::collections::HashMap;

pub struct Term{
    a: f64,
    b: f64,
    c: f64,
}

pub(super) fn check_equation_degree(terms: &mut HashMap<i32, f64>) -> Result<i32, String>{

    terms.retain(|_, & mut value| value != 0.0 || value != -0.0); // ne garde que les elements de la map avant une value != de 0

    if let Some(&key) = terms.keys().max(){
        return Ok(key);
    }
    return Err("no key??".to_string());
}

pub(super) fn fill_equation_struct(terms: &HashMap<i32, f64>) -> Term{

    let eq_terms = Term {
        a : get_coef(terms, 2),
        b : get_coef(terms, 1),
        c : get_coef(terms, 0),
    };
    println!("a: {}  b: {}  c: {}", eq_terms.a, eq_terms.b, eq_terms.c);
    eq_terms
}

fn get_coef(terms: &HashMap<i32, f64>, exponent_to_find: i32) -> f64{

    if let Some((_exponent, _coef)) = terms.iter().find(|(&_exponent, &_coef)| _exponent == exponent_to_find){
        *_coef
    } else{
        0.0
    }
}

pub(super) fn resolve_degre_0(term: &Term){
    // println!(dbg"resolve degre 0")
    println!("The solution is:\n{}", term.c);
}

pub(super) fn resolve_degre_1(term: &Term){
    // println!("resolve degre 1")
    let result = - term.c / term.b;
    println!("The solution is:\n{}", result);
}

pub(super) fn resolve_degre_2(term: &Term){
    // println!("resolve degre 2")
    let discriminant = compute_discriminant(&term);

        println!("discri: {}", discriminant);
    if discriminant > 0.0 {
        let solution = compute_positive_discriminant_solution(&term, discriminant);
        println!("Discriminant is strictly positive, the two solutions are:\n{}\n{}", solution.0, solution.1);
    
    } else if discriminant == 0.0 {
        let solution = compute_zero_discrimiannt_solution(term);
        println!("Discriminant is strictly equal to zero, the solution is:\n{}", solution);

    } else if discriminant < 0.0 {
        println!("Discriminant is strictly negative, no real solution, only complex.");

    }
}

fn compute_discriminant(term: &Term)-> f64{ //d =  b^2 - 4ac

    (term.b * term.b) - (4.0 * term.a * term.c)
}

fn compute_positive_discriminant_solution(term: &Term, discriminant: f64)-> (f64, f64){

    let sqrt_result = discriminant.sqrt();

    let positive_solution = (-term.b + sqrt_result)/(2.0 *term.a);
    let negative_solution = (-term.b - sqrt_result)/(2.0 *term.a);

    (positive_solution, negative_solution)
}

fn compute_zero_discrimiannt_solution(term: &Term)->f64{
    (-term.b) / (term.a*term.a)
}