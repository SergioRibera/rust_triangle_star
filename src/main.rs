use std::io::{self, Write, stdin, Read};

// **********************************
// *                                *
// *      The Tree Algorithm        *
// *                                *
// **********************************
// private function
fn generate_tree(n: i32) -> Result<(), &'static str> {
    // if n is 0, throw error
    if n <= 0 {
        return Err("The size of the tree must be greater than 0");
    }
    // Run from 0 to n, for made n lines
    for i in 0..n {
        // calculate space from left
        let n_spaces = n - i - 1;
        // calculate stars to draw
        let n_stars = (i * 2) + 1;
        // generate spaces string
        // first create a vector with n_spaces length filled by " " space character
        let spaces = vec![" "; n_spaces as usize].join("");
        // generate star string
        // first create a vector with n_stars length filled by "*" star character
        let stars = vec!["*"; n_stars as usize].join("");

        // show line on console
        println!("{spaces}{stars}")
    }
    Ok(())
}

fn main() -> Result<(), &'static str> {
    //
    // Question
    //
    // show into console the question for input the tree size
    print!("Type the size of the tree: ");
    // flush the last print output
    io::stdout().flush().unwrap();

    //
    // User Answer
    //
    // Create empty 1024 buffer
    let mut buf = [0u8; 1024];

    // read from standar input and save input on buf, its return a length of user input
    if let Ok(c) = stdin().read(&mut buf) {
        // if user input length is more than 0, continue
        if c > 1 {
            // convert array buf to String and unwrap if is valid
            let answer = String::from_utf8(buf[..c - 1].to_vec()).unwrap();
            // Try parse String to i32, its return a Result, if Result is Ok(n) we call to
            // algorithm function
            if let Ok(n) = answer.parse::<i32>() {
                // call the function and implicitly return its result
                generate_tree(n)
            } else {
                // If cannot parse, we trow an error
                Err("Not valid number detected")
            }
        } else {
            // If user not write any or only press enter,
            // we trow an error
            Err("You need write something")
        }
    } else {
        // If user not write any, we trow an error
        Err("An error ocurred")
    }
}
