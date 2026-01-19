use pcre2::bytes::Regex;
use std::{error::Error, fs, path::PathBuf};

use crate::types::YaccError; 

pub fn run(filename: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Ran lexer!");
    let code = fs::read_to_string(filename)?;
    let code_bytes = code.as_bytes();
    let mut tokens = vec![];

    let regex = Regex::new(concat!(
        r"(?P<identifier>[a-zA-Z_]\w*\b)|",
        r"(?P<constant>[0-9]+\b)|",
        r"(?P<symbol>[;\(\){}])|",
        r"(?P<whitespace>\s+)|",
        r"(?P<error>.)"))?;

    for result in regex.captures_iter(code_bytes) {
        let caps = result?;
        if let Some(identifier) = caps.name("identifier") {
            let identifier = str::from_utf8(identifier.as_bytes())?;
            println!("Found identifier: {}", identifier);
            tokens.push(identifier.to_owned());
        }
        else if let Some(constant) = caps.name("constant") {
            let constant = str::from_utf8(constant.as_bytes())?;
            println!("Found constant: {}", constant);
            tokens.push(constant.to_owned());
        } else if let Some(symbol) = caps.name("symbol") {
            let symbol = str::from_utf8(symbol.as_bytes())?;
            println!("Found symbol: {}", symbol);
            tokens.push(symbol.to_owned());         
        } else if let Some(erroneous) =  caps.name("error") {
            println!("Found error: {}", str::from_utf8(erroneous.as_bytes())?); 
            return Err(Box::new(YaccError::InvalidToken));            
        }
    }

    Ok(tokens)
}
