use std::{
    io::{self, Read, Write},
    str::FromStr,
};

// import library for print with colors
use colored::Colorize;

// public function
// If the value is required, this function will keep asking until a valid answer is obtained
//
// T is generic type
// validation is a function, usually we get from validation.rs file
pub fn ask<T>(question: &str, require: bool, validation: impl Fn(&str) -> bool) -> Result<T, T::Err>
where
    // T need implement FromStr trait
    T: FromStr,
{
    // define mutable validation variable
    let mut is_valid = false;
    // store answer
    let mut answer = String::new();
    // define for all function the standar input stream
    let mut stdin = io::stdin();
    // define prefix for question, by default is a one "Tab"
    let mut prefix = String::from_utf8("\t".into()).unwrap();

    // If this question is required, add to prefix a indication for user, by default (*) with
    // colors
    if require {
        prefix.push_str(format!("({}) ", "*".red()).as_str());
    }

    while !is_valid {
        // write to output buffer the prefix and question formatted
        print!("{}{}: ", prefix, question.bright_blue());
        // flush the output for show the question
        io::stdout().flush().unwrap();
        // create empty buffer
        let mut buf = [0u8; 1024];
        // get the standar input (user input) and put on buf
        if let Ok(c) = stdin.read(&mut buf) {
            // if the user write anything, the length of the input is more than 0
            if c > 0 {
                // Convert buffer on String
                answer = String::from_utf8(buf[..c - 1].to_vec()).unwrap();
                // call to validation funciton and pass the answer value
                // set valid equal to validation result or check if is not required for ignore
                is_valid = validation(answer.trim()) || !require;
            } else {
                if !require {
                    break;
                }
                // show on console a message for rewrite answer
                println!("\t{}", "You need write something".magenta());
            }
        } else {
            if !require {
                break;
            }
            // error while read the input
            println!("\tAn error ocurred");
        }
    }
    // return generic type from answer using the trait method (from_str)
    T::from_str(&answer)
}
