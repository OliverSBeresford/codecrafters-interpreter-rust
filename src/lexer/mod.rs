pub mod token;
pub mod scanner;

pub use scanner::{scan, TokenArray};
pub use token::{Keyword, Literal, Token, TokenType};
