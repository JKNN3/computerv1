
// struct term{
//     quadratic: <f64>,
//     linear: <f64>,
//     constant:<f64>,
// };

use std::env;
// use std::io;


mod parser;
use parser::parser;
// use macros::REGEX_GET_TERMS;
// use std::collections::HashMap;


fn main() -> Result<(), String>{

    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args.iter().map(String::as_str).collect();

    parser(input);

    Ok(())
}
