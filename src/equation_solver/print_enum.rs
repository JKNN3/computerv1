
/*  when the enum is called, .display() print the 
    corresponding message   */

use std::collections::HashMap;

pub enum Print<'a>{
    PolynomialDegree(i32),
    PolynomialDegreeError,
    Solution(f64),
    NoSolution,
    SolutionPositiveDiscriminant(f64, f64),
    SolutionNullDiscriminant(f64),
    SolutionNegativeDiscriminant(f64, f64),
    ReducedForm(& 'a HashMap<i32, f64>),
}

impl Print<'_>{
    pub fn display(&self){
        match self{
            Print::PolynomialDegree(degree) => println!("Polynomial degree: {}", degree),
            Print::PolynomialDegreeError => eprintln!("The polynomial degree is strictly greater than 2, I can't solve."),   
            Print::Solution(nb) => println!("The solution is:\n{}", nb),
            Print::NoSolution => println!("There is no solution."),
            Print::SolutionPositiveDiscriminant(nb1, nb2) => println!("Discriminant is strictly positive, the two solutions are:\n{}\n{}", nb1, nb2),
            Print::SolutionNullDiscriminant(nb) => println!("Discriminant is strictly equal to zero, the solution is:\n{}", nb),
            Print::SolutionNegativeDiscriminant(nb1, nb2) => println!("Discriminant is strictly negative, the two complex solutions are:\n{} + {}i\n{} - {}i", nb1, nb2, nb1, nb2),
            Print::ReducedForm(terms) => {
                let mut to_print: String = String::new();

                for (key, value) in *terms {
                    if *value != 0.0 {
                        to_print += &format!("{} {}*X^{} ",
                            if *value > 0.0 && !to_print.is_empty() { "+" } else { "-" },
                            value.abs(), key);
                    }
                }
            
                if let Some(first_char) = to_print.chars().next() {
                    if first_char == '+' || first_char == '-' {
                        to_print.remove(0);
                    }
                }
                to_print = to_print.trim_end().to_string();
            
                println!("Reduced form: {} = 0", to_print);
                
            }
        }
    }
}
