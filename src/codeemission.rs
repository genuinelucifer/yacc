use std::{error::Error, fs::File, io::{BufWriter, Write}, path::PathBuf};

use crate::types::{YaccError, assemblytypes::*};

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
    write!(writer, "    .globl ")?;

    #[allow(unused_mut)]
    let mut name = func.name;

    #[cfg(target_os = "macos")]
    {
        // Add underscrore to function names on macos
         name = "_".to_string() + &name;
    }

    writeln!(writer, "{}", &name)?;
    writeln!(writer, "{}:", &name)?;

    // Create function stack
    writeln!(writer, "    pushq %rbp")?;
    writeln!(writer, "    movq %rsp, %rbp")?;

    let indent = 4;
    for instruction in func.instructions {
        write_instruction(instruction, indent, writer)?;
    }


    Ok(())
}

fn write_instruction(inst: Instruction, indent: usize, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
    let indent = " ".repeat(indent);
    match inst {
        Instruction::Mov(op1, op2) => {
            write!(writer, "{indent}movl ")?;
            write_operand(op1, writer)?;
            write!(writer, ", ")?;
            write_operand(op2, writer)?;
            writeln!(writer, "")?;
        },
        Instruction::Ret => {
            writeln!(writer, "{indent}movq %rbp, %rsp")?;
            writeln!(writer, "{indent}popq %rbp")?;
            writeln!(writer, "{indent}ret")?
        },
        Instruction::AllocateStack(stack_size) => {
            write!(writer, "{indent}subq ")?;
            write_operand(Operand::Imm(stack_size), writer)?;
            writeln!(writer, ", %rsp")?;
        },
        Instruction::Unary(uop, op) => {
            match uop {
                AssemblyUnaryOperator::Neg => write!(writer, "{indent}negl ")?,
                AssemblyUnaryOperator::Not => write!(writer, "{indent}notl ")?,
            }
            write_operand(op, writer)?;
            writeln!(writer, "")?;
        },
    }

    Ok(())
}

fn write_operand(op: Operand, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
    match op {
        Operand::Imm(val) => write!(writer, "${}", val)?,
        Operand::Reg(Register::EAX) => write!(writer, "%eax")?,
        Operand::Reg(Register::R10D) => write!(writer, "%r10d")?,
        Operand::Stack(offset) => write!(writer, "-{}(%rbp)", offset)?,
        _ => {
            return Err(Box::new(YaccError::InvalidAssemblyOperand(op)));
        },
    }

    Ok(())
}
