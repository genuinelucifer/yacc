use std::path::PathBuf;
use thiserror::Error;

use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("stage_group")
        .args(["lex", "parse", "tacky", "codegen", "assembly"])
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
    tacky: bool,
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
    Tacky,
    CodeGen,
    CodeEmission,
    All,
}

pub fn stage_to_run_from_cli(cli: &Cli) -> StagesToRun {
    if cli.lex {
        StagesToRun::Lexer
    } else if cli.parse {
        StagesToRun::Parser
    } else if cli.tacky {
        StagesToRun::Tacky
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

pub mod tackytypes {
    #[derive(Debug, Clone)]
    pub enum TackyValue {
        Constant(i32),
        Variable(String),
    }

    #[derive(Debug)]
    pub enum TackyUnaryOperator {
        Complement,
        Negation,
    }

    #[derive(Debug)]
    pub enum TackyInstruction {
        Return(TackyValue),
        // Operator, src, dst
        TackyUnary(TackyUnaryOperator, TackyValue, TackyValue),
    }
    
    #[derive(Debug)]
    pub struct TackyFunction {
        pub name: String,
        pub body: Vec<TackyInstruction>,
    }

    #[derive(Debug)]
    pub struct TackyProgram(pub TackyFunction);
}

pub mod assemblytypes {
    #[derive(Debug, Clone)]
    pub enum Register {
        EAX,
        R10D,
    }

    #[derive(Debug, Clone)]
    pub enum Operand {
        Imm(i32),
        Reg(Register),
        Pseudo(String),
        Stack(i32),
    }

    #[derive(Debug)]
    pub enum AssemblyUnaryOperator {
        Neg,
        Not,
    }

    #[derive(Debug)]
    pub enum Instruction {
        Mov(Operand, Operand),
        Unary(AssemblyUnaryOperator, Operand),
        AllocateStack(i32),
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
