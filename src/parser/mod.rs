mod parser;
mod print_library;

use std::env;
use std::collections::HashMap;
use parser::get_terms_map;
use parser::check_input_format;
// use parser_utils::*; // * pour pouvoir utiliser toutes les fonctions publiques du fichier parser_utils


pub fn parser() -> Result<HashMap<i32, f64>, String>{

    let input: Vec<String> = env::args().collect();
    if input.len() != 2 {
        return Err("1 arg  like \"nX^0 = nX^1\", print usage".to_string())
    }

    let input: &str = &input[1].replace(" ", ""); 
    if !check_input_format(input){
        return Err("wrong format string".to_string())
    }

    get_terms_map(input)

}