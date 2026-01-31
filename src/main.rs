use clap::Parser;
use std::error::Error;
use std::process::Command;

mod types;
use types::{Cli, StagesToRun, stage_to_run_from_cli};

mod codeemission;
mod codegen;
mod lexer;
mod parser;
mod tacky;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let stages = stage_to_run_from_cli(&cli);
    let mut filename = cli.filename;

    let tokens = lexer::run(filename.clone())?;
    println!("Tokens Generated: {:?}", tokens);
    if stages < StagesToRun::Parser {
        return Ok(());
    }

    let ast = parser::run(tokens)?;
    println!("AST generated: {:?}", ast);
    if stages < StagesToRun::Tacky {
        return Ok(());
    }

    let tacky = tacky::run(ast)?;
    println!("Tacky generated: {:?}", tacky);
    if stages < StagesToRun::CodeGen {
        return Ok(());
    }
/*
    let assembly = codegen::run(tacky);
    println!("Assembly generated: {:?}", assembly);
    if stages < StagesToRun::CodeEmission {
        return Ok(());
    }

    filename.set_extension("s");
    codeemission::run(assembly, filename.clone())?;
    println!("The assembly is written to {filename:?}");
    if stages < StagesToRun::All {
        return Ok(());
    }

    // Run gcc to make an executable for the program
    let mut executable = filename.clone();
    executable.set_extension("");

    // gcc $filename -o $executable
    let _ = Command::new("gcc").args([filename.to_str().unwrap(), "-o", executable.to_str().unwrap()]).output()?;
*/
    Ok(())
}
