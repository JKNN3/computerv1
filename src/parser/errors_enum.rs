pub const REGEX_CHECK_INPUT_FORMAT: &str = r"^((0)|(([-+]?\d+(\.\d+)?\*X\^\d+)([-+]\d+(\.\d+)?\*X\^\d+)*))=((0)|(([-+]?\d+(\.\d+)?\*X\^\d+)([-+]\d+(\.\d+)?\*X\^\d+)*))$";

/*  when the enum is called , .get() give the
formatted string containing the error to return */

pub enum Errors<'a> {
    PrintUsage,
    WrongInputFormat(&'a str),
    SplitFailed,
    ParseCoefFailed(&'a str),
    ParseExponentFailed(&'a str),
}
impl Errors<'_> {
    pub fn get(&self) -> String {
        match *self {
            Errors::PrintUsage => {
                eprintln!("Every term must respect the form a∗X^p.\nTry '5*X^0 + 4*X^1 - 9.3*X^2 = 1*X^0'");
                format!("Wrong usage")
            }
            Errors::WrongInputFormat(input) => {
                eprintln!("Every term must respect the form a∗X^p.\nTry '5*X^0 + 4*X^1 - 9.3*X^2 = 1*X^0'");
                format!("Wrong input format : {input}")
            }
            Errors::SplitFailed => format!("Failed to get split values"),
            Errors::ParseCoefFailed(coef) => format!("Failed to parse coef '{}'", coef),
            Errors::ParseExponentFailed(exp) => format!("Failed to parse exponent '{}'", exp),
        }
    }
}
