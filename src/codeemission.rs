use std::{error::Error, fs::File, io::{BufWriter, Write}, path::PathBuf};

use crate::types::assemblytypes::*;

pub fn run(program: AssemblyProgram, filename: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    write_function(program.0, &mut writer)?;

    #[cfg(target_os = "linux")]
    {
        // Add this add the end of assembly in linux to force non-executable stack
         writeln!(writer, ".section .note.GNU-stack,\"\",@progbits")?;
    }

    writer.flush()?;
    Ok(())
}

fn write_function(func: FunctionDefinition, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
    write!(writer, "    .global ")?;

    #[allow(unused_mut)]
    let mut name = func.name;

    #[cfg(target_os = "macos")]
    {
        // Add underscrore to function names on macos
         name = "_".to_string() + &name;
    }

    writeln!(writer, "{}", &name)?;
    writeln!(writer, "{}:", &name)?;

    let indent = 4;
    for instruction in func.instructions {
        write_instruction(instruction, indent, writer)?;
    }


    Ok(())
}

fn write_instruction(inst: Instruction, indent: usize, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
    write!(writer, "{}", " ".repeat(indent))?;
    match inst {
        Instruction::Mov(op1, op2) => {
            write!(writer, "movl ")?;
            write_operand(op1, writer)?;
            write!(writer, ", ")?;
            write_operand(op2, writer)?;
            writeln!(writer, "")?;
        },
        Instruction::Ret => writeln!(writer, "ret")?,
    }

    Ok(())
}

fn write_operand(op: Operand, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
    match op {
        Operand::Imm(val) => write!(writer, "${}", val)?,
        Operand::Register => write!(writer, "%eax")?,
    }

    Ok(())
}
