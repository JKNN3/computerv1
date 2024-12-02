
use std::collections::HashMap;

pub(super) fn check_equation_degree(terms: &mut HashMap<i32, f64>) -> Result<i32, String>{

    dbg!(&terms);
    terms.retain(|_, & mut value| value != 0.0 || value != -0.0); // ne garde que les elements de la map avant une value != de 0
    dbg!(&terms);

    if let Some(&key) = terms.keys().max(){
        return Ok(key);
    }
    return Err("no key??".to_string());
}