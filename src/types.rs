use std::path::PathBuf;

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