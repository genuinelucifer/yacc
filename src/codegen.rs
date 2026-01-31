use crate::types::{assemblytypes::*, parsertypes::*};

pub fn run(ast: Program) -> AssemblyProgram {
    let function_def = parse_function(ast.0);
    AssemblyProgram(function_def)
}

fn parse_function(function: FunctionSignature) -> FunctionDefinition {
    let name = function.name;
    let instructions = parse_statement(function.body);

    FunctionDefinition { name, instructions }
}

fn parse_statement(stmt: Statement) -> Vec<Instruction> {
    match stmt {
        Statement::ReturnStatement(Expression::Constant(value)) => {
            return vec![
                Instruction::Mov(Operand::Imm(value), Operand::Register),
                Instruction::Ret,
            ];
        },
        _ => unimplemented!("Codegen for this statement not implemented yet!")
    }
}
