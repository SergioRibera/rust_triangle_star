// define modules as private
//
// define the modules to rust
mod interactive;
// define the modules to rust
mod validation;

// We expose all the content of the modules as if they were our own,
// this will allow us to make use of them in a direct way, e.g.
//
// // pub use interactive::*;
// use user_input::ask;
//
// // without pub use interactive::*;
// use user_input::interactive::ask;
//
pub use interactive::*;
pub use validation::*;
