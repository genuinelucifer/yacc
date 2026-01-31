use std::path::PathBuf;
use thiserror::Error;

use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("stage_group")
        .args(["lex", "parse", "codegen", "assembly"])
))]
pub struct Cli {
    // filename, required argument
    pub filename: PathBuf,

    // All the stage related arguments
    #[arg(long)]
    lex: bool,
    #[arg(long)]
    parse: bool,
    #[arg(long)]
    codegen: bool,
    #[arg(long)]
    assembly: bool,

    // Any extra flags to be put here
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum StagesToRun {
    Lexer,
    Parser,
    CodeGen,
    CodeEmission,
    All,
}

pub fn stage_to_run_from_cli(cli: &Cli) -> StagesToRun {
    if cli.lex {
        StagesToRun::Lexer
    } else if cli.parse {
        StagesToRun::Parser
    } else if cli.codegen {
        StagesToRun::CodeGen
    } else if cli.assembly {
        StagesToRun::CodeEmission
    } else {
        StagesToRun::All
    }
}

#[derive(Debug, Error)]
pub enum YaccError {
    #[error("Invalid token found")]
    InvalidToken(String),

    #[error("Unexpected program end")]
    UnexpectedEnd,

    #[error("Expected a different token")]
    UnexpectedToken(lexertypes::LexerTokens),
}

pub mod lexertypes {

    #[derive(Debug, PartialEq)]
    pub enum IdentifierTokens {
        UserIdentifier(String),
        Return,
        Int,
        Void,
    }

    #[derive(Debug, PartialEq)]
    pub enum ConstantTokens {
        IntegerConstant(i32),
    }

    #[derive(Debug, PartialEq)]
    pub enum SymbolTokens {
        LParen,
        RParen,
        LCurly,
        RCurly,
        SemiColon,
        Decrement,
        Negation,
        Complement,
    }

    #[derive(Debug, PartialEq)]
    pub enum LexerTokens {
        Identifier(IdentifierTokens),
        LineComment(String),
        MultiLineComment(String),
        Constant(ConstantTokens),
        Symbol(SymbolTokens)
    }

}

pub mod parsertypes {
    #[derive(Debug)]
    pub enum UnaryOperator {
        Complement,
        Negation,
    }

    #[derive(Debug)]
    pub enum Expression {
        Constant(i32),
        UnaryExp(UnaryOperator, Box<Expression>),
    }

    #[derive(Debug)]
    pub enum Statement {
        ReturnStatement(Expression)
    }

    #[derive(Debug)]
    pub struct FunctionSignature {
        pub name: String,
        pub body: Statement,
    }

    #[derive(Debug)]
    pub struct Program(pub FunctionSignature);
}

pub mod assemblytypes {
    #[derive(Debug)]
    pub enum Operand {
        Imm(i32),
        Register
    }

    #[derive(Debug)]
    pub enum Instruction {
        Mov(Operand, Operand),
        Ret,
    }

    #[derive(Debug)]
    pub struct FunctionDefinition {
        pub name: String,
        pub instructions: Vec<Instruction>,
    }

    #[derive(Debug)]
    pub struct AssemblyProgram(pub FunctionDefinition);
}
