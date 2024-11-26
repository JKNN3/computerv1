// use std::env;

// enum EquationTermKind{
//     Quadratic(f64, i32),    // Coefficient (f64), Exponent (i32)  
//     Linear(f64),            // Coefficient (f64)
//     Constant(f64),          // Constant (f64)
// }


// fn print_env_args(){

//     let args: Vec<String> = env::args().collect(); // get les arguments passés en param du pgm

//     let args_iter = args.iter(); // crée un iterateur

//     for arg in args_iter { // itere dessus
//         println!("{arg}"); // print
//     }
// }

// let f = s.parse::<f32>().unwrap(); // convert str to type (f32)

fn get_coef(equation_vec: &str){
    
    let mut sign : bool = true;
    let mut coef : f64 = 0;

    for str in equation_vec{
        
        match str {}
        "*" => break;
        "+" => sign = true;
        "-" => sign = false;
        _ =>{ 
            let number = 
        }
    }
}


fn main(){

    let equation_string = String::from("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0");
    dbg!(&equation_string);

    // let splitted_equation_string = equation_string.split(" ");
    let splitted_equation_vec: Vec<&str> = equation_string.split(" ").collect();



    for term in splitted_equation_vec{
        println!("{term}");
        
    }


}