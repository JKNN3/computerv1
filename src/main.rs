
// struct term{
//     quadratic: <f64>,
//     linear: <f64>,
//     constant:<f64>,
// };

// use std::io;


mod parser;
use crate::parser::parser;
// use std::collections::HashMap;
// use parser::parser;
// use macros::REGEX_GET_TERMS;
// use std::collections::HashMap;


fn main() -> Result<(), String>{

    let terms = parser()?;
    dbg!(terms);
    Ok(())
}
