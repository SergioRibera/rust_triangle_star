
// **********************************
// *                                *
// *      The Tree Algorithm        *
// *                                *
// **********************************
// private function
fn generate_tree(n: i32) -> Result<(), String> {
    // if n is 0, throw error
    if n <= 0 {
        return Err("The size of the tree must be greater than 0".to_string());
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

fn main() {
}
