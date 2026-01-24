use pcre2::bytes::Regex;
use std::{error::Error, fs, path::PathBuf};

use crate::types::{lexertypes::*, YaccError};

pub fn run(filename: PathBuf) -> Result<Vec<LexerTokens>, Box<dyn Error>> {
    let code = fs::read_to_string(filename)?;
    let code_bytes = code.as_bytes();
    let mut tokens = vec![];

    let regex = Regex::new(concat!(
        r"(?P<identifier>[a-zA-Z_]\w*\b)|",
        r"(?P<constant>[0-9]+\b)|",
        r"(?P<symbol>[;\(\){}])|",
        r"(?P<comment>//.*\n)|",
        r"(?P<whitespace>\s+)|",
        r"(?P<error>.+\b)"))?;

    for result in regex.captures_iter(code_bytes) {
        let caps = result?;
        if let Some(identifier) = caps.name("identifier") {
            let identifier = str::from_utf8(identifier.as_bytes())?;

            let tk = match identifier {
                 "return" => LexerTokens::Identifier(IdentifierTokens::Return),
                 "int" => LexerTokens::Identifier(IdentifierTokens::Int),
                 "void" => LexerTokens::Identifier(IdentifierTokens::Void),
                 x => LexerTokens::Identifier(IdentifierTokens::UserIdentifier(x.to_string())),
            };
            tokens.push(tk);
        }
        else if let Some(comment) = caps.name("comment") {
            let comment = str::from_utf8(comment.as_bytes())?;
            tokens.push(LexerTokens::LineComment(comment.to_string()));
        }
        else if let Some(constant) = caps.name("constant") {
            let constant = str::from_utf8(constant.as_bytes())?;
            let value = constant.parse()?;
            tokens.push(LexerTokens::Constant(ConstantTokens::IntegerConstant(value)));
        }
        else if let Some(symbol) = caps.name("symbol") {
            let symbol = str::from_utf8(symbol.as_bytes())?;

            let tk = match symbol {
                ";" => LexerTokens::Symbol(SymbolTokens::SemiColon),
                "(" => LexerTokens::Symbol(SymbolTokens::LParen),
                ")" => LexerTokens::Symbol(SymbolTokens::RParen),
                "{" => LexerTokens::Symbol(SymbolTokens::LCurly),
                "}" => LexerTokens::Symbol(SymbolTokens::RCurly),
                _ => std::unreachable!(),
            };
            tokens.push(tk);
        }
        else if let Some(erroneous) =  caps.name("error") {
            let err = str::from_utf8(erroneous.as_bytes())?;
            println!("Found error: {}", err);
            return Err(Box::new(YaccError::InvalidToken(err.to_string())));
        }
    }

    Ok(tokens)
}
