
use regex::Regex;
use self::macros;
use macros::REGEX_CHECK_INPUT;
// use macros::REGEX_GET_TERMS;
// use std::collections::HashMap;


pub fn parser(input : Vec<&str>) -> Result<(), String>{

    if input.len() != 2 {
        return Err("wrong usage".to_string())
    }
    if !check_input_format(&input[1]){
        return Err("aaa".to_string())
    }

    Ok(())
    // let arg = arg.replace(" ", "");
    // check_input_format(&arg);
    // println!("{}", arg);

    // let eq_vec : Vec::<&str> = arg.split("=").collect();
    // let equation_left_side_terms = capture_equation_terms(&eq_vec[0]);
    // let equation_right_side_terms = capture_equation_terms(&eq_vec[1]);
    
    

    // fill_equation_terms_map();
}


fn check_input_format(input: &str) -> bool{

    let regex_check_input = Regex::new(REGEX_CHECK_INPUT).unwrap();
    regex_check_input.is_match(&input)
}


// fn capture_equation_terms(input : &str) -> Vec::<String>{ // put in a vector every term entity with "n * X^y" format

//     let regex_get_terms = Regex::new(REGEX_GET_TERMS).unwrap();
//     let input = input.replace("X^", "");

//     let mut vec_of_captured_terms = Vec::<String>::new();

//     for matches in regex_get_terms.find_iter(&input){
//         vec_of_captured_terms.push(matches.as_str().to_string());
//     }
//     vec_of_captured_terms
// }



// fn parse_term(term_splitted: Vec<&str>) -> Result<(i32, f64), String> {

//     let coef = term_splitted[1].parse::<f64>().map_err(|_| format!("Failed to parse coefficient '{}'", term_splitted[1]))?;
//     let exponent = term_splitted[0].parse::<i32>().map_err(|_| format!("Failed to parse exponent '{}'", term_splitted[0]))?;

//     Ok((exponent, coef))
// }


// fn add_terms_to_map(terms: &mut HashMap<i32, f64>, vector_of_terms: Vec::<&str>, sign: f64) -> bool{

//     for term in vector_of_terms{
//         match parse_term(term.split("*").collect()){
//             Ok((exponent, coef)) => {
//                     terms.entry(exponent)
//                         .and_modify(|e| *e += coef * sign)
//                         .or_insert_with(|| coef * sign);
//             }
//             Err(err) => {
//                 println!("{}", err);
//                 return false
//             }
//         }
//     }
//     return true
// }

// fn fill_equation_terms_map(right_side_terms: Vec::<&str>, left_side_terms: Vec::<&str>){

//     let mut terms: HashMap<i32, f64>= HashMap::new();

//     add_terms_to_map(&mut terms, left_side_terms, 1.0)
//     add_terms_to_map(&mut terms, right_side_terms, -1.0)
// }