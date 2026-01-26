use clap::Parser;
use std::error::Error;
use std::process::Command;

mod types;
use types::{Cli, StagesToRun, stage_to_run_from_cli};

mod codeemission;
mod codegen;
mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let stages = stage_to_run_from_cli(&cli);
    let mut filename = cli.filename;

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

    filename.set_extension("s");
    codeemission::run(assembly, filename.clone())?;
    if stages < StagesToRun::All {
        return Ok(());
    }
    println!("The assembly is written to {filename:?}");

    // Run gcc to make an executable for the program
    let mut executable = filename.clone();
    executable.set_extension("");

    // gcc $filename -o $executable
    let _ = Command::new("gcc").args([filename.to_str().unwrap(), "-o", executable.to_str().unwrap()]).output()?;

    Ok(())
}
