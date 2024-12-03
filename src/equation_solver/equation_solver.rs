
use std::collections::HashMap;
use super::print_librairy::Print;


pub struct Term{
    a: f64,
    b: f64,
    c: f64,
}

pub(super) fn check_equation_degree(terms: &mut HashMap<i32, f64>) -> Result<i32, String> {

    terms.retain(|_, & mut value| value != 0.0 || value != -0.0); // ne garde que les elements de la map avant une value != de 0

    if let Some(&key) = terms.keys().max(){
        return Ok(key);
    }
    return Err("no key??".to_string());
}

pub(super) fn fill_equation_struct(terms: &HashMap<i32, f64>) -> Term {

    let eq_terms = Term {
        a : get_coef(terms, 2),
        b : get_coef(terms, 1),
        c : get_coef(terms, 0),
    };
    eq_terms
}

fn get_coef(terms: &HashMap<i32, f64>, exponent_to_find: i32) -> f64 {

    if let Some((_exponent, _coef)) = terms.iter().find(|(&_exponent, &_coef)| _exponent == exponent_to_find){
        *_coef
    } else {
        0.0
    }
}

pub(super) fn resolve_degre_0(term: &Term) {

    Print::Solution(term.c).display();
}

pub(super) fn resolve_degre_1(term: &Term) {

    let result = - term.c / term.b;
    Print::Solution(result).display();
}

pub(super) fn resolve_degre_2(term: &Term) {

    let discriminant = compute_discriminant(&term);

    if discriminant > 0.0 {
        let solution = compute_positive_discriminant_solution(&term, discriminant);
        Print::SolutionPositiveDiscriminant(solution.0, solution.1).display();
    
    } else if discriminant == 0.0 {
        let solution = compute_zero_discrimiannt_solution(term);
        Print::SolutionNullDiscriminant(solution).display();

    } else if discriminant < 0.0 {
        Print::SolutionNegativeDiscriminant.display();
    }
}

fn compute_discriminant(term: &Term) -> f64 { //d =  b^2 - 4ac

    (term.b * term.b) - (4.0 * term.a * term.c)
}

fn compute_positive_discriminant_solution(term: &Term, discriminant: f64) -> (f64, f64) {

    let sqrt_result = discriminant.sqrt();

    let positive_solution = (-term.b + sqrt_result)/(2.0 *term.a);
    let negative_solution = (-term.b - sqrt_result)/(2.0 *term.a);

    (positive_solution, negative_solution)
}

fn compute_zero_discrimiannt_solution(term: &Term) ->f64 {
    (-term.b) / (term.a*term.a)
}