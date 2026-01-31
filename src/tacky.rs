use crate::types::{parsertypes::*, tackytypes::*};
use std::boxed::Box;
use std::error::Error;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn run(ast: Program) -> Result<TackyProgram, Box<dyn Error>> {
    let tacky_func = parse_function(ast.0)?;

    Ok(TackyProgram(tacky_func))
}

fn parse_function(func: FunctionSignature) -> Result<TackyFunction, Box<dyn Error>> {
    let name = func.name;
    let body = parse_statement(func.body)?;

    Ok(TackyFunction { name, body })
}

fn parse_statement(stmt: Statement) -> Result<Vec<TackyInstruction>, Box<dyn Error>> {
    match stmt {
        Statement::ReturnStatement(exp) => {
            let mut insts = vec![];
            let last_var = parse_expression(&exp, &mut insts)?;
            insts.push(TackyInstruction::Return(last_var));
            Ok(insts)
        }
    }
}

fn parse_expression(
    exp: &Expression,
    insts: &mut Vec<TackyInstruction>,
) -> Result<TackyValue, Box<dyn Error>> {
    match exp {
        Expression::Constant(constant) => Ok(TackyValue::Constant(*constant)),
        Expression::UnaryExp(uop, rexp) => {
            let tuop = match uop {
                UnaryOperator::Complement => TackyUnaryOperator::Complement,
                UnaryOperator::Negation => TackyUnaryOperator::Negation,
            };
            let src = parse_expression(rexp, insts)?;
            let dst = TackyValue::Variable(create_temp_var());
            insts.push(TackyInstruction::TackyUnary(tuop, src, dst.clone()));
            Ok(dst)
        }
    }
}

// Atomics are thread-safe and can be stored in statics
static NUM_VARS: AtomicI32 = AtomicI32::new(-1);

fn create_temp_var() -> String {
    // fetch_add increments the value and returns the PREVIOUS value
    // "Ordering" defines how memory synchronization happens
    let current = NUM_VARS.fetch_add(1, Ordering::SeqCst);
    format!("tmp.{}", current + 1)
}
