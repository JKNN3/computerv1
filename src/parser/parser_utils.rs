
use regex::Regex;
use std::collections::HashMap;
use super::macros::REGEX_CHECK_INPUT;

pub(super) fn check_input_format(input: &str) -> bool{
    println!("check_input_format");
    let regex_check_input = Regex::new(REGEX_CHECK_INPUT).unwrap();
    regex_check_input.is_match(&input)

}

pub(super) fn get_terms_map(input: &str) -> Result<HashMap<i32, f64>, String>{
    println!("get_terms_map");

    let input = input.replace("X^", "");
    let eq_vec : Vec::<&str> = input.split("=").collect();
    println!("eq vec 1: {}, eq vec 2: {}", eq_vec[0], eq_vec[1]);
    let left_side_terms = capture_equation_terms(&eq_vec[0]);
    let right_side_terms = capture_equation_terms(&eq_vec[1]);

    fill_equation_terms_map(left_side_terms, right_side_terms)

}

fn capture_equation_terms(input : &str) -> Vec::<&str>{ // put in a vector every term entity with "n * X^y" format
    println!("capture_equation_terms");

    if input.contains("+") || input.contains("-"){
        input.split( |c| { c == '+' || c == '-'}).collect()
    }
    else{
        vec![input]
    }
}

fn fill_equation_terms_map(right_side_terms: Vec::<&str>, left_side_terms: Vec::<&str>) -> Result<HashMap<i32, f64>, String>{
    println!("fill_equation_terms_map");

    let mut terms: HashMap<i32, f64>= HashMap::new();

    add_terms_to_map(&mut terms, left_side_terms, -1.0)?;
    add_terms_to_map(&mut terms, right_side_terms, 1.0)?;

    Ok(terms)
}


fn add_terms_to_map(terms: &mut HashMap<i32, f64>, vector_of_terms: Vec::<&str>, sign: f64) -> Result<(),String>{
    println!("add_terms_to_map");

    for term in vector_of_terms{ if term.contains("*"){
    println!("{}", term);
        let term = term.split("*").collect();
        match parse_term(term){
            Ok((exponent, coef)) => {
                println!("exp{} coef:{}", exponent,coef);
                terms.entry(exponent)
                    .and_modify(|e| *e += coef * sign)
                    .or_insert_with(|| coef * sign);
            }
            Err(err) => {return Err(err)}
        }
    }}
    Ok(())
}

fn parse_term(term_splitted: Vec<&str>) -> Result<(i32, f64), String> {
    println!("parse_term");

    let coef = term_splitted[0].parse::<f64>().map_err(|_| format!("Failed to parse coefficient '{}'", term_splitted[1]))?;
    let exponent = term_splitted[1].parse::<i32>().map_err(|_| format!("Failed to parse exponent '{}'", term_splitted[0]))?;
    println!("coef: {coef}, exp: {exponent}");
    Ok((exponent, coef))
}