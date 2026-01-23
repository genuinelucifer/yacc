use std::error::Error;

use crate::types::{LexerTokens, IdentifierTokens, ConstantTokens, SymbolTokens, YaccError};


pub fn run(tokens: Vec<LexerTokens>) -> Result<(), Box<dyn Error>> {
    println!("Ran parser!");
    Ok(())
}
