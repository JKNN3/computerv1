
/*  when the enum is called, .display() print the 
    corresponding message   */

pub enum Print{
    PolynomialDegree(i32),
    PolynomialDegreeError,
    ReducedFormDegre0(f64),
    ReducedFormDegre1(f64, f64),
    ReducedFormDegre2(f64, f64, f64),
    Solution(f64),
    SolutionPositiveDiscriminant(f64, f64),
    SolutionNullDiscriminant(f64),
    SolutionNegativeDiscriminant,
}


impl Print{
    pub fn display(&self){
        match *self{
            Print::PolynomialDegree(degree) => println!("Polynomial degree: {}", degree),
            Print::PolynomialDegreeError => eprintln!("The polynomial degree is strictly greater than 2, I can't solve."),
            Print::ReducedFormDegre0(c) => println!("Reduced form: {}{} * X^0 = 0", 
                                                        if c > 0.0 { "" } else { "- " }, c.abs()),
            Print::ReducedFormDegre1(b, c) => println!("Reduced form: {}{} * X^0 {} {} * X^1 = 0",
                                                        if c > 0.0 { "" } else { "- " }, c.abs(),
                                                        if b > 0.0 { "+" } else { "-" }, b.abs()),
            Print::ReducedFormDegre2(a, b, c) => println!("Reduced form: {}{} * X^0 {} {} * X^1 {} {} * X^2 = 0",
                                                        if c > 0.0 { "" } else { "- " }, c.abs(),
                                                        if b > 0.0 { "+" } else { "-" }, b.abs(), 
                                                        if a > 0.0 { "+" } else { "-" }, a.abs()),          
            Print::Solution(nb) => println!("The solution is:\n{}", nb),
            Print::SolutionPositiveDiscriminant(nb1, nb2) => println!("Discriminant is strictly positive, the two solutions are:\n{}\n{}", nb1, nb2),
            Print::SolutionNullDiscriminant(nb) => println!("Discriminant is strictly equal to zero, the solution is:\n{}", nb),
            Print::SolutionNegativeDiscriminant => println!("Discriminant is strictly negative, no real solution, only complex."),
        }
    }
}