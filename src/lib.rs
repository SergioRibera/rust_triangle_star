// on this file we will expose all functions, structs, macros and more, like as library

pub mod lang;
pub mod user_input;

// generally the prelude module is usually the module in which the most used functions,
// submodules and struct are exported, to give a little less verbose access when someone uses our library.
pub mod prelude {
    pub use super::lang::*;
    pub use super::user_input::*;
}

// **********************************
// *                                *
// *      The Tree Algorithm        *
// *                                *
// **********************************
// private function
pub fn generate_tree(
    n: i32,
    padding: i32,
    sentence: &[&'static str],
) -> Result<String, &'static str> {
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
    let mut res: Vec<String> = vec![];
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

        // add line to result vec
        res.push(format!("{spaces}{stars}"));
    }
    // return Ok with all elements on array joined by new line (\n)
    Ok(res.join("\n"))
}
