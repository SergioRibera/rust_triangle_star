// import library for print with colors
use colored::Colorize;
use rust_triangle_star::{
    generate_tree,
    prelude::{is_language_valid, LANGUAGES},
    user_input::{accept, ask, is_n, is_number, MyBool},
};

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
