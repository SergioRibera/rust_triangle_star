use std::str::FromStr;

// static array with Positive posible answers
const ACCEPT: &[&str] = &["Yes", "yes", "Y", "y"];
// static array with Negative posible answers, empty is "enter"
const DECLINE: &[&str] = &["No", "no", "N", "n", "\n", ""];

// Simple struct with only one anonymous parameter
#[derive(Debug)]
pub struct MyBool(pub bool);

// Implemented "FromStr" trait (traits are like interfaces in other languages)
// This implementation allow convert &str to MyBool (my own struct)
impl FromStr for MyBool {
    type Err = String;

    // implementation of the method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // check if answer exists on Positive answers
        if ACCEPT.contains(&s) {
            Ok(Self(true))
        }
        // check if answer exists on Negative answers
        else if DECLINE.contains(&s) {
            Ok(Self(false))
        } else {
            // Throw error
            Err(format!("\"{s}\" Not recognized"))
        }
    }
}

// check if the answer is positive
pub fn accept(s: &str) -> bool {
    // check if is empty and convert &str to MyBool and map the result, if the result si Ok, we
    // return the anonymous parameter which is called 0 (like an array) else if is an Error we map
    // the output to false by default
    !s.is_empty() || MyBool::from_str(s.trim()).map_or(false, |x| x.0)
}

// verify is number
pub fn is_number(s: &str) -> bool {
    // try to convert from &str to i32 (int) and made same mapping in accept
    //
    // the function map_or is like that:
    //
    // match s.trim().parse::<i32>() {
    //     Ok(_) => true,
    //     Err(_) => false
    // }
    //
    s.trim().parse::<i32>().map_or(false, |_| true)
}

// Validation for N (tree length)
// verify if the number input is valid (is number && is > 0)
pub fn is_n(s: &str) -> bool {
    // check if is number and if the parse result is more than 0, is valid
    is_number(s) && s.trim().parse::<i32>().unwrap_or(0) > 0
}
