use super::errors_enum::Errors;
use super::errors_enum::REGEX_CHECK_INPUT_FORMAT;
use regex::Regex;
use std::collections::HashMap;

/*  check the input format with a regular expression    */
pub(super) fn check_input_format(input: &str) -> bool {
    let regex_check_input = Regex::new(REGEX_CHECK_INPUT_FORMAT).unwrap();
    regex_check_input.is_match(&input)
}

/*  convert the string input to a map where the key is the
exponent and the value is the coefficient */
pub(super) fn get_terms_map(input: &str) -> Result<HashMap<i32, f64>, String> {
    let input = format_input(input);
    let eq_vec: Vec<&str> = input.split("=").collect();

    if let (Some(a), Some(b)) = (eq_vec.get(0), eq_vec.get(1)) {
        let left_side_terms = capture_equation_terms(a);
        let right_side_terms = capture_equation_terms(b);

        fill_equation_terms_map(left_side_terms, right_side_terms)
    } else {
        Err(Errors::SplitFailed.get())
    }
}

/*  put in a vector every term entity with "n * X^y" format
 */
fn capture_equation_terms(input: &str) -> Vec<&str> {
    if input.contains("+") || input.contains("-") {
        input.split(" ").collect()
    } else {
        vec![input]
    }
}

/*  add all the terms to the map, inverting sign for the
right side put every terms on the same side */
fn fill_equation_terms_map(
    right_side_terms: Vec<&str>,
    left_side_terms: Vec<&str>,
) -> Result<HashMap<i32, f64>, String> {
    let mut terms: HashMap<i32, f64> = HashMap::new();

    add_terms_to_map(&mut terms, right_side_terms, 1.0)?;
    add_terms_to_map(&mut terms, left_side_terms, -1.0)?;

    Ok(terms)
}

/*  fill a map with terms entity "n * X^y" where key = y
and value = n, add coef to corresponding key value
if already exists,or add a new entry (to reduce the
equation at the same time)  */
fn add_terms_to_map(
    terms: &mut HashMap<i32, f64>,
    vector_of_terms: Vec<&str>,
    sign: f64,
) -> Result<(), String> {
    for term in vector_of_terms {
        if term.contains("*") {
            let term = term.split("*").collect();
            match parse_term(term) {
                Ok((exponent, coef)) => {
                    terms
                        .entry(exponent)
                        .and_modify(|e| *e += coef * sign)
                        .or_insert_with(|| coef * sign);
                }
                Err(err) => return Err(err),
            }
        }
    }
    Ok(())
}

/*  convert input strings (coef & exp) to target number types
 */
fn parse_term(term: Vec<&str>) -> Result<(i32, f64), String> {
    let coef = term[0]
        .parse::<f64>()
        .map_err(|_| Errors::ParseCoefFailed(term[0]).get())?;
    let exponent = term[1]
        .parse::<i32>()
        .map_err(|_| Errors::ParseExponentFailed(term[1]).get())?;
    Ok((exponent, coef))
}

/*  putting spaces before the signs to split on the space
and keep the sign  */
fn format_input(input: &str) -> String {
    input.replace("X^", "").replace("+", " +").replace("-", " -")
}
