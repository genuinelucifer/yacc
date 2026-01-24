use clap::Parser;
use std::error::Error;

mod types;
use types::{Cli, StagesToRun, stage_to_run_from_cli};

mod codeemission;
mod codegen;
mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let stages = stage_to_run_from_cli(&cli);
    let filename = cli.filename;

    let tokens = lexer::run(filename.clone())?;
    if stages < StagesToRun::Parser {
        return Ok(());
    }
    println!("Tokens Generated: {:?}", tokens);

    let ast = parser::run(tokens)?;
    if stages < StagesToRun::CodeGen {
        return Ok(());
    }
    println!("AST generated: {:?}", ast);

    let assembly = codegen::run(ast);
    if stages < StagesToRun::CodeEmission {
        return Ok(());
    }
    println!("Assembly generated: {:?}", assembly);

    let _ = codeemission::run()?;
    if stages < StagesToRun::All {
        return Ok(());
    }

    println!("The filename is {filename:?} amd the stages are: {stages:?}");
    Ok(())
}
