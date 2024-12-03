
pub enum Print{
    PolynomialDegree(i32),
    PolynomialDegreeError,
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
            Print::Solution(nb) => println!("The solution is:\n{}", nb),
            Print::SolutionPositiveDiscriminant(nb1, nb2) => println!("Discriminant is strictly positive, the two solutions are:\n{}\n{}", nb1, nb2),
            Print::SolutionNullDiscriminant(nb) => println!("Discriminant is strictly equal to zero, the solution is:\n{}", nb),
            Print::SolutionNegativeDiscriminant => println!("Discriminant is strictly negative, no real solution, only complex."),
        }
    }
}