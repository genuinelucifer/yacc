use std::{error::Error, vec::IntoIter};

use crate::types::{YaccError, lexertypes::*, parsertypes::*};


pub fn run(tokens: Vec<LexerTokens>) -> Result<Program, Box<dyn Error>> {
    let mut iter = tokens.into_iter();
    let function = parse_function(&mut iter)?;

    // All the remaining program MUST be comments
    while let Some(ntoken) = iter.next() {
        match ntoken {
            LexerTokens::LineComment(_) | LexerTokens::MultiLineComment(_) => continue,
            _ => return Err(Box::new(YaccError::UnexpectedToken(ntoken))),
        }
    }

    Ok(Program(function))
}

fn expect_token(tk: LexerTokens, iter: &mut IntoIter<LexerTokens>) -> Result<(), Box<dyn Error>> {
    let ntoken = iter.next();
    match ntoken {
        Some(ntoken) if ntoken == tk => Ok(()),
        Some(LexerTokens::LineComment(_)) => expect_token(tk, iter),
        Some(LexerTokens::MultiLineComment(_)) => expect_token(tk, iter),
        Some(ntoken) => Err(Box::new(YaccError::UnexpectedToken(ntoken))),
        None => Err(Box::new(YaccError::UnexpectedEnd)),
    }
}

fn parse_function(iter: &mut IntoIter<LexerTokens>) -> Result<FunctionSignature, Box<dyn Error>> {
    expect_token(LexerTokens::Identifier(IdentifierTokens::Int), iter)?;

    let name;
    match iter.next() {
        Some(LexerTokens::Identifier(IdentifierTokens::UserIdentifier(fn_name))) => name = fn_name,
        Some(ntoken) => return Err(Box::new(YaccError::UnexpectedToken(ntoken))),
        None => return Err(Box::new(YaccError::UnexpectedEnd))
    }

    expect_token(LexerTokens::Symbol(SymbolTokens::LParen), iter)?;
    expect_token(LexerTokens::Identifier(IdentifierTokens::Void), iter)?;
    expect_token(LexerTokens::Symbol(SymbolTokens::RParen), iter)?;
    expect_token(LexerTokens::Symbol(SymbolTokens::LCurly), iter)?;

    let body = parse_statement(iter)?;

    expect_token(LexerTokens::Symbol(SymbolTokens::RCurly), iter)?;
    Ok(FunctionSignature { name,  body })
}

fn parse_statement(iter: &mut IntoIter<LexerTokens>) -> Result<Statement, Box<dyn Error>> {
    expect_token(LexerTokens::Identifier(IdentifierTokens::Return), iter)?;

    let exp = parse_expression(iter)?;

    expect_token(LexerTokens::Symbol(SymbolTokens::SemiColon), iter)?;
    Ok(Statement::ReturnStatement(exp))
}

fn parse_expression(iter: &mut IntoIter<LexerTokens>) -> Result<Expression, Box<dyn Error>> {
    match iter.next() {
        Some(LexerTokens::Symbol(SymbolTokens::LParen)) => {
            let exp = parse_expression(iter)?;
            expect_token(LexerTokens::Symbol(SymbolTokens::RParen), iter)?;
            Ok(exp)
        },
        Some(LexerTokens::Constant(ConstantTokens::IntegerConstant(constant)))=> Ok(Expression::Constant(constant)),
        Some(LexerTokens::Symbol(SymbolTokens::Complement)) => {
            let exp = parse_expression(iter)?;
            Ok(Expression::UnaryExp(UnaryOperator::Complement, Box::new(exp)))
        },
        Some(LexerTokens::Symbol(SymbolTokens::Negation)) => {
            let exp = parse_expression(iter)?;
            Ok(Expression::UnaryExp(UnaryOperator::Negation, Box::new(exp)))
        },
        Some(ntoken) => Err(Box::new(YaccError::UnexpectedToken(ntoken))),
        None => Err(Box::new(YaccError::UnexpectedEnd))
    }
}
