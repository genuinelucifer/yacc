use crate::types::{assemblytypes::*, tackytypes::*};

pub fn run(ast: TackyProgram) -> AssemblyProgram {
    let function_def = parse_function(ast.0);

    AssemblyProgram(function_def)
}

fn parse_function(function: TackyFunction) -> FunctionDefinition {
    let name = function.name;

    let mut instructions = vec![];
    for inst in function.body {
        instructions.extend(parse_instruction(inst));
    }

    // Now a pass to update all Pseudo registers to Stack values
    let stack_size = update_pseudo_operand_to_stack(&mut instructions);

    let mut valid_insts = Vec::with_capacity(1+instructions.len());
    valid_insts.push(Instruction::AllocateStack(stack_size));

    // Now a pass to fix all the invalid statements
    fix_invalid_instructions(instructions, &mut valid_insts);

    FunctionDefinition { name, instructions: valid_insts }
}

fn fix_invalid_instructions(instructions: Vec<Instruction>, valid_insts: &mut Vec<Instruction>) {
    for inst in instructions {
        match inst {
            Instruction::Mov(Operand::Stack(off1), Operand::Stack(off2)) => {
                valid_insts.push(Instruction::Mov(Operand::Stack(off1), Operand::Reg(Register::R10D)));
                valid_insts.push(Instruction::Mov(Operand::Reg(Register::R10D), Operand::Stack(off2)));
            },
            _ => valid_insts.push(inst),
        }
    }
}

fn update_pseudo_operand_to_stack(instructions: &mut Vec<Instruction>) -> i32 {
    let mut st = stack::StackCreator::new();

    for inst in instructions {
        match inst {
            Instruction::Mov(src, dst) => {
                if let Operand::Pseudo(p) = src {
                    *src = st.get_or_create_32bit(p); 
                }
                if let Operand::Pseudo(p) = dst {
                    *dst = st.get_or_create_32bit(p); 
                }
            },
            Instruction::Unary(_, dst) => {
                if let Operand::Pseudo(p) = dst {
                    *dst = st.get_or_create_32bit(p); 
                }
            },
            _ => {},
        }
    }

    st.stack_size()
}

fn parse_instruction(inst: TackyInstruction) -> Vec<Instruction> {
    match inst {
        TackyInstruction::Return(val) => {
            let op = get_operand_from_value(val);
            return vec![
                Instruction::Mov(op, Operand::Reg(Register::EAX)),
                Instruction::Ret,                
            ];
        },
        TackyInstruction::TackyUnary(tuop, src, dst) => {
            let src = get_operand_from_value(src);
            let dst = get_operand_from_value(dst);
            let tuop = match tuop {
                TackyUnaryOperator::Complement => AssemblyUnaryOperator::Not,
                TackyUnaryOperator::Negation => AssemblyUnaryOperator::Neg,
            };
            return vec![
                Instruction::Mov(src, dst.clone()),
                Instruction::Unary(tuop, dst),
            ];
        },
        _ => unimplemented!("Cannot handle the instruction {inst:?} yet!")
    }
}

fn get_operand_from_value(val: TackyValue) -> Operand {
    match val {
        TackyValue::Constant(constant) => Operand::Imm(constant),
        TackyValue::Variable(ident) => Operand::Pseudo(ident),
    }
}

mod stack {
    use std::collections::HashMap;

    use crate::types::assemblytypes::Operand;

    // Private struct to create Stack operands
    pub struct StackCreator {
        op_to_stack: HashMap<String, i32>,
        tot_bytes_allocated: i32,
    }

    impl StackCreator {
        pub fn new() -> Self {
            Self {op_to_stack: HashMap::new(), tot_bytes_allocated: 0}
        }

        pub fn get_or_create_32bit(&mut self, tempname: &str) -> Operand {
            if let Some(offset) = self.op_to_stack.get(tempname) {
                Operand::Stack(*offset)
            } else {
                self.tot_bytes_allocated += 4;
                self.op_to_stack.insert(tempname.to_owned(), self.tot_bytes_allocated);
                Operand::Stack(self.tot_bytes_allocated)
            }
        }

        pub fn stack_size(&self) -> i32 {
            self.tot_bytes_allocated
        }
    }
}