mod parser;
mod errors_enum;

use std::env;
use std::collections::HashMap;
use parser::get_terms_map;
use parser::check_input_format;
use errors_enum::Errors;

/*  check input and give a map contianing my terms
    in the form (key = exponent, value = coef)      */
pub fn parser() -> Result<HashMap<i32, f64>, String>{

    let input: Vec<String> = env::args().collect();
    if input.len() != 2 {
        return Err(Errors::PrintUsage.get())
    }

    let input: &str = &input[1].replace(" ", ""); 
    if !check_input_format(input){
        return Err(Errors::WrongInputFormat(input).get())
    }

    get_terms_map(input)

}