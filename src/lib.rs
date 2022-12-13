// on this file we will expose all functions, structs, macros and more, like as library

pub mod lang;
pub mod user_input;

// generally the prelude module is usually the module in which the most used functions,
// submodules and struct are exported, to give a little less verbose access when someone uses our library.
pub mod prelude {
    pub use super::lang::*;
    pub use super::user_input::*;
}
