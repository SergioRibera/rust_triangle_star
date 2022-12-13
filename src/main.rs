use lang::{is_language_valid, LANGUAGES};
use user_input::{accept, ask, is_n, is_number, MyBool};

// import library for print with colors
use colored::Colorize;

// define the modules to rust compiler
mod lang;
mod user_input;

// **********************************
// *                                *
// *      The Tree Algorithm        *
// *                                *
// **********************************
// private function
fn generate_tree(n: i32, padding: i32, sentence: &[&'static str]) -> Result<(), &'static str> {
    // if n is 0, throw error
    if n <= 0 {
        // & is a reference
        // * use for dereference
        // the first * is for dereference &[] array
        // the second * is for derefence &'static str
        // and the & on first is for made a new reference to &'static str
        //
        // It for fix the clippy borrow-deref-ref warning
        return Err(&**sentence.get(3).unwrap());
    }
    // Run from 0 to n, for made n lines
    for i in 0..n {
        // calculate space from left
        let n_spaces = (n - i - 1) + padding;
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
    let mut cont = true;
    let lang = ask::<String>("Choose your language [en|es]", true, is_language_valid)
        .map_or("en".to_string(), |n| n);
    let sentences = LANGUAGES.iter().find(|l| l.0 == lang).unwrap();
    let padding = ask::<i32>(sentences.1.first().unwrap(), true, is_number).map_or(20, |n| n);
    // continue infinite if the user want
    while cont {
        // ask to user how much big want a tree
        let n = ask::<i32>(sentences.1.get(1).unwrap(), false, is_n).map_or(5, |n| n);
        // call to Algorithm with a default padding
        generate_tree(n, padding, sentences.1)?;
        let ok = ask::<MyBool>(sentences.1.get(2).unwrap(), true, accept)
            .map_err(|e| println!("\n\t{}", e.magenta()))
            .unwrap_or(MyBool(false));
        cont = ok.0;
    }
    Ok(())
}
