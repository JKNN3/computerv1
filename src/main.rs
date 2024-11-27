




// fn print_env_args(){

//     let args: Vec<String> = env::args().collect(); // get les arguments passés en param du pgm

//     let args_iter = args.iter(); // crée un iterateur

//     for arg in args_iter { // itere dessus
//         println!("{arg}"); // print
//     }
// }

// let f = s.parse::<f32>().unwrap(); // convert str to type (f32)


// fn get coef
/*
    Split sur les espaces

*/


// fn get_coef(equation_vec: &str){
    
//     let mut sign : bool = true;
//     let mut coef : f64 = 0;

//     for str in equation_vec{
        
//         match str {
//             "*" => break;
//             "+" => sign = true;
//             "-" => sign = false;
//             _ =>{ 
//                 match str.parse::<f64>().unwrap(){
//                     Ok(number) => coef += number;
//                     Err(e) => println!("Not a number or a managed operator {}", e);
//                     break;
//                 }
//             }
//         }
//     }
//     if sign == false
//         coef = -coef
//     coef
// }

// use std::env;
mod macros;
use regex::Regex;
use macros::REGEX_CHECK_INPUT;
use macros::REGEX_GET_TERMS;
// use macros::REGEX_GET_COEF_AND_EXPOSANT;

enum EquationTermKind{
    Quadratic(f64, i32),    // Coefficient (f64), Exponent (i32)  
    Linear(f64),            // Coefficient (f64)
    Constant(f64),          // Constant (f64)
}

fn check_input_format(input: &str){ // parse input

    let regex_check_input = Regex::new(REGEX_CHECK_INPUT).unwrap();
    assert!(regex_check_input.is_match(&input), "The input have the wrong format"); // + print le format a "utiliser"

}

fn capture_equation_terms(input : &str) -> Vec::<&str>{ // parse input

    let regex_get_terms = Regex::new(REGEX_GET_TERMS).unwrap();

    let mut vec_of_captured_terms = Vec::<&str>::new();

    for matches in regex_get_terms.find_iter(input){
        vec_of_captured_terms.push(matches.as_str());
    }
    vec_of_captured_terms
}

fn fill_equation_terms_vector(right_side_terms: Vec::<&str>, left_side_terms: Vec::<&str>){

    let mut equation_terms_vec = Vec::<EquationTermKind>::new();

    for term in right_side_terms {
        let term_splitted = term.split("*");
        let coefficient = term_splitted[0].parse::<f64>()
            .unwrap_or_else(panic!("number not compatible"));
        match term_splitted[1]{
            "X^0"=>{ equation_terms_vec.push(EquationTermKind::Constant(coefficient));}
            "X^1"=>{ equation_terms_vec.push(EquationTermKind::Linear(coefficient));}
            _ => {equation_terms_vec.push(EquationTermKind::Quadratic(coefficient, 2));}
        }

    }
    dbg!(equation_terms_vec)

}

fn main(){

    let equation_string = String::from("5 * X^2 + 5.1 * X^3 = 2 * X^6").replace(" ", "");
    dbg!(&equation_string);

    check_input_format(&equation_string);

    let eq_vec : Vec::<&str> = equation_string.split("=").collect();

    // println!("{}", eq_vec[0]); // capturer les groupes et remplir mon enum pour la gauche
    // println!("{}", eq_vec[1]); // capturer les groupes et remplir mon enum en les mettant a gauche en inversant leur signe
    let equation_left_side_terms = capture_equation_terms(&eq_vec[0]);

    // dbg!(equation_left_side_terms);
    let equation_right_side_terms = capture_equation_terms(&eq_vec[1]);

    fill_equation_terms_vector(equation_left_side_terms, equation_right_side_terms);
    // dbg!(equation_right_side_terms);
    // reduire
    // calculer le delta
    // mettre tout ca dans une loop ua cas ou on ait plusieurs inputs
    // faire les prints

    // capture_equation_terms(&equation_string)
    // let splitted_equation_string = equation_string.split(" ");
    // let splitted_equation_vec: Vec<&str> = equation_string.split(" ").collect();

    // for term in splitted_equation_vec{
        // println!("{term}");
        // 
    // }


}