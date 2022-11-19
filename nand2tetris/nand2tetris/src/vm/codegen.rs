use super::instructions::{VMInstruction, Segment, Arithmetic};
use crate::assembler::{Instruction, ComputeOp, ComputeFields, JumpOp, DestOp};

pub struct CodeGenerator {
    pub static_namespace: String,
    pub label_counter: usize,
}

impl CodeGenerator {
    pub fn new(static_namespace: String) -> CodeGenerator {
        CodeGenerator {
            static_namespace,
            label_counter: 0,
        }
    }

    pub fn translate(&mut self, vm_instruction: &VMInstruction) -> Vec<Instruction> {
        let mut instructions = vec![];
        match &vm_instruction {
            VMInstruction::Push(segment, value) => {
                match segment {
                    Segment::Constant => {
                        // load constant
                        instructions.extend(self.segment_to_address_instruction(segment, *value));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(false),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }));
                        instructions.extend(self.gen_push_d()) // push D
                    }
                    Segment::Temp => {
                        // load constant
                        instructions.extend(self.segment_to_address_instruction(segment, *value));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }));
                        instructions.extend(self.gen_push_d()) // push D
                    }
                    Segment::Static => {
                        instructions.extend(self.segment_to_address_instruction(segment, *value));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }));
                        instructions.extend(self.gen_push_d()) // push D
                    }
                    Segment::Pointer => {
                        instructions.extend(vec![
                            match value {
                                0 => Instruction::LabeledAddress("THIS".to_string()),
                                1 => Instruction::LabeledAddress("THAT".to_string()),
                                _ => panic!("pointer can only be used with 1 and 0"),
                            },
                            Instruction::Compute(ComputeFields {
                                compute_op: ComputeOp::A(true),
                                jump_op: JumpOp::Nothing,
                                destination_op: DestOp::D,
                            }),
                        ]);
                        instructions.extend(self.gen_push_d()) // push D
                    }
                    _ => {
                        // all other cases are loading from memory segments
                        instructions.extend(self.segment_to_address_instruction(segment, *value));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::DPlusA(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::A,
                        }));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }));
                        instructions.extend(self.gen_push_d()) // push D
                    }
                }
            }
            VMInstruction::Pop(segment, value) => match segment {
                Segment::Static => {
                    instructions.extend(self.segment_to_address_instruction(segment, *value));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }));
                    instructions.push(Instruction::LabeledAddress("R13".to_string()));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                    instructions.extend(self.pop_to_r13_pointer())
                }
                Segment::Pointer => {
                    match value {
                        0 => instructions.push(Instruction::LabeledAddress("THIS".to_string())),
                        1 => instructions.push(Instruction::LabeledAddress("THAT".to_string())),
                        _ => panic!("pointer can only be 0 or 1"),
                    }
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }));
                    instructions.push(Instruction::LabeledAddress("R13".to_string()));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                    instructions.extend(self.pop_to_r13_pointer())
                }
                Segment::Temp => {
                    instructions.extend(self.segment_to_address_instruction(segment, *value));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }));
                    instructions.push(Instruction::LabeledAddress("R13".to_string()));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                    instructions.extend(self.pop_to_r13_pointer())
                }
                _ => {
                    instructions.extend(self.segment_to_address_instruction(segment, *value));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DPlusA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }));
                    instructions.push(Instruction::LabeledAddress("R13".to_string()));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                    instructions.extend(self.pop_to_r13_pointer())
                }
            },
            VMInstruction::Arithmetic(operation) => match operation {
                Arithmetic::Add => {
                    instructions.extend(self.pop_to_d());
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DPlusA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Sub => {
                    instructions.extend(self.pop_to_d());
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::AMinusD(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Neg => {
                    instructions.push(Instruction::LabeledAddress("SP".to_string()));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DecA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::A,
                    }));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::MinusA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Eq => instructions.extend(self.true_or_false(JumpOp::Equal)),
                Arithmetic::Gt => instructions.extend(self.true_or_false(JumpOp::Greater)),
                Arithmetic::Lt => instructions.extend(self.true_or_false(JumpOp::Lower)),
                Arithmetic::And => {
                    instructions.extend(self.pop_to_d());
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DAndA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Or => {
                    instructions.extend(self.pop_to_d());
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DOrA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Not => {
                    instructions.push(Instruction::LabeledAddress("SP".to_string()));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DecA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::A,
                    }));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::NotA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
            },
            VMInstruction::Label(label) => instructions.push(Instruction::Label(label.to_string())),
            VMInstruction::GoTo(label) => {
                instructions.push(Instruction::LabeledAddress(label.to_string()));
                instructions.push(Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::Zero,
                    jump_op: JumpOp::Unconditional,
                    destination_op: DestOp::Nothing,
                }));
            }
            VMInstruction::IfGoTo(label) => {
                instructions.push(Instruction::LabeledAddress("SP".to_string()));
                instructions.push(Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::DecA(true),
                    jump_op: JumpOp::Nothing,
                    destination_op: DestOp::M,
                }));
                instructions.push(Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::A(true),
                    jump_op: JumpOp::Nothing,
                    destination_op: DestOp::A,
                }));
                instructions.push(Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::A(true),
                    jump_op: JumpOp::Nothing,
                    destination_op: DestOp::D,
                }));
                instructions.push(Instruction::LabeledAddress(label.to_string()));
                instructions.push(Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::D,
                    jump_op: JumpOp::NotEqual,
                    destination_op: DestOp::Nothing,
                }));
            }
            VMInstruction::Function(function_name, number_of_locals) => {
                // todo: this can be optimized easily
                instructions.extend(vec![
                    Instruction::Label(function_name.to_string()),
                    Instruction::Address(*number_of_locals),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                    Instruction::LabeledAddress("SP".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::A,
                    }),
                ]);
                for _ in 0..*number_of_locals {
                    instructions.extend(vec![
                        Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::Zero,
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::M,
                        }),
                        Instruction::Compute(ComputeFields {
                            // last one always redundant
                            compute_op: ComputeOp::IncA(false),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::A,
                        }),
                    ]);
                }
                instructions.extend(vec![
                    Instruction::LabeledAddress("SP".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DPlusA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }),
                ]);
            }
            VMInstruction::Call(call_data) => {
                self.label_counter += 1;
                let formatted_return = format!("{}$ret{}", call_data.from, self.label_counter);
                instructions.extend(vec![
                    Instruction::LabeledAddress(formatted_return.clone()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                ]);
                instructions.extend(self.push_d());
                for segment in ["LCL", "ARG", "THIS", "THAT"] {
                    instructions.extend(vec![
                        Instruction::LabeledAddress(segment.to_string()),
                        Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }),
                    ]);
                    instructions.extend(self.push_d());
                }
                let offset = call_data.arguments + 5;
                instructions.extend(vec![
                    Instruction::LabeledAddress("SP".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                    Instruction::LabeledAddress("LCL".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }),
                    Instruction::Address(offset),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DMinusA(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                    Instruction::LabeledAddress("ARG".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }),
                    Instruction::LabeledAddress(call_data.target.clone()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::Zero,
                        jump_op: JumpOp::Unconditional,
                        destination_op: DestOp::Nothing,
                    }),
                    Instruction::Label(formatted_return),
                ]);
            }
            VMInstruction::Return => {
                instructions.extend(vec![
                    Instruction::LabeledAddress("LCL".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                    Instruction::LabeledAddress("R13".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }),
                    // "@5", "D=A", "@R13", "D=M-D", "@R14", "M=D", // R14 = R13 - 5
                    // "@5", "D=A", "@R13", "D=M-D", "A=D", "A=M", "D=M", "@R14", "M=D", // R14 = R13 - 5
                    // "@5", "D=A", "@R13", "D=M-D", "A=D", "D=M", "@R14", "M=D", // R14 = R13 - 5
                    Instruction::Address(5),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                    Instruction::LabeledAddress("R13".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::AMinusD(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::A,
                    }),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                    Instruction::LabeledAddress("R14".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }),
                    // ..
                    Instruction::LabeledAddress("SP".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DecA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::A,
                    }),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                    Instruction::LabeledAddress("ARG".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::A,
                    }),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }),
                    // SP = ARG+1
                    Instruction::LabeledAddress("ARG".to_string()),
                    Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::IncA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }),
                ]);
                instructions.extend(self.assign_d_to_segment_memory("SP".to_string()));
                let dec_r13_to_d = || {
                    vec![
                        Instruction::LabeledAddress("R13".to_string()),
                        Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::DecA(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::AM,
                        }),
                        Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }),
                    ]
                };
                for segment_name in ["THAT", "THIS", "ARG", "LCL"] {
                    // segment = *(--R13)
                    instructions.extend(dec_r13_to_d());
                    instructions.extend(self.assign_d_to_segment_memory(segment_name.to_string()));
                }
                // "@R14", "A=M;JMP", // goto *R14
                instructions.push(Instruction::LabeledAddress("R14".to_string()));
                instructions.push(Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::A(true),
                    jump_op: JumpOp::Unconditional,
                    destination_op: DestOp::A,
                }));
            }
            VMInstruction::Comment(_) => (),
        }
        instructions
    }

    fn assign_d_to_segment_memory(&self, segment_name: String) -> Vec<Instruction> {
        vec![
            Instruction::LabeledAddress(segment_name),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::D,
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
        ]
    }

    fn push_d(&self) -> Vec<Instruction> {
        vec![
            Instruction::LabeledAddress("SP".to_string()),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::IncA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::DecA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::D,
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
        ]
    }

    fn true_or_false(&mut self, jump_op: JumpOp) -> Vec<Instruction> {
        let mut instructions = vec![];
        instructions.extend(self.pop_to_d());
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::AMinusD(true),
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::D,
        }));
        self.label_counter += 1;
        let true_label = format!("true.{}", self.label_counter);
        instructions.push(Instruction::LabeledAddress(true_label.clone()));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::D,
            jump_op,
            destination_op: DestOp::Nothing,
        }));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::Zero,
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::D,
        }));
        self.label_counter += 1;
        let end_label = format!("end.{}", self.label_counter);
        instructions.push(Instruction::LabeledAddress(end_label.clone()));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::Zero,
            jump_op: JumpOp::Unconditional,
            destination_op: DestOp::Nothing,
        }));
        instructions.push(Instruction::Label(true_label));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::MinusOne,
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::D,
        }));
        instructions.push(Instruction::Label(end_label));
        instructions.push(Instruction::LabeledAddress("SP".to_string()));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::DecA(true),
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::A,
        }));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::D,
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::M,
        }));
        instructions
    }

    fn pop_to_d(&self) -> Vec<Instruction> {
        vec![
            Instruction::LabeledAddress("SP".to_string()),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::DecA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::D,
            }),
            Instruction::LabeledAddress("SP".to_string()),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::DecA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
        ]
    }

    fn segment_to_address_instruction(
        &mut self,
        segment: &Segment,
        offset: u16,
    ) -> Vec<Instruction> {
        let mut instructions = vec![match &segment {
            Segment::Static => Instruction::LabeledAddress(
                self.static_namespace.clone() + "." + &offset.to_string(),
            ),
            Segment::Temp => Instruction::Address(5 + offset),
            _ => Instruction::Address(offset),
        }];
        match segment {
            Segment::Constant => (),
            Segment::Temp => (),
            Segment::Static => (),
            _ => {
                instructions.push(Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::A(false),
                    jump_op: JumpOp::Nothing,
                    destination_op: DestOp::D,
                }));
            }
        }
        match segment {
            Segment::Local => instructions.push(Instruction::LabeledAddress("LCL".to_string())),
            Segment::Argument => instructions.push(Instruction::LabeledAddress("ARG".to_string())),
            Segment::This => instructions.push(Instruction::LabeledAddress("THIS".to_string())),
            Segment::That => instructions.push(Instruction::LabeledAddress("THAT".to_string())),
            _ => (),
        }
        instructions
    }

    fn gen_push_d(&self) -> Vec<Instruction> {
        vec![
            // @SP
            Instruction::LabeledAddress("SP".to_string()),
            // A=M
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
            // M=D
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::D,
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
            // @SP
            Instruction::LabeledAddress("SP".to_string()),
            // M=M+1
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::IncA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
        ]
    }

    fn pop_to_r13_pointer(&self) -> Vec<Instruction> {
        vec![
            Instruction::LabeledAddress("SP".to_string()),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::DecA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::D,
            }),
            Instruction::LabeledAddress("R13".to_string()),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::D,
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::instructions::*;
    use crate::assembler::generate_instruction;

    fn assert_instructions(expected: &Vec<&str>, vm_instruction: VMInstruction) {
        let instructions = CodeGenerator::new("Test".to_string()).translate(&vm_instruction);
        let smallest = expected.len().min(instructions.len());
        for index in 0..smallest {
            let str_instruction = generate_instruction(&instructions[index]);
            assert_eq!(expected[index], str_instruction, "index = {}", index);
        }
        assert_eq!(expected.len(), instructions.len());
    }

    #[test]
    fn generate_push_constant_42() {
        assert_instructions(
            &vec![
                "@42", "D=A", // can we just @SP?
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::Constant, 42),
        );
    }

    #[test]
    fn generate_push_local_13() {
        assert_instructions(
            &vec![
                "@13",   // 13 offset
                "D=A",   // store offset in D
                "@LCL",  // LOCAL base addr
                "A=D+M", // sum offset
                "D=M",   // read D = RAM[LOCAL + offset]
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::Local, 13),
        );
    }

    #[test]
    fn generate_push_static_20() {
        assert_instructions(
            &vec![
                "@Test.20", "D=M", // program name is Test
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::Static, 20),
        );
    }

    #[test]
    fn generate_push_argument_13() {
        assert_instructions(
            &vec![
                "@13",   // load offset
                "D=A",   // store offset in D
                "@ARG",  // THIS base addr
                "A=D+M", // sum offset
                "D=M",   // read D = RAM[THIS + offset]
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::Argument, 13),
        );
    }

    #[test]
    fn generate_push_this_19() {
        assert_instructions(
            &vec![
                "@19",   // load offset
                "D=A",   // store offset in D
                "@THIS", // THIS base addr
                "A=D+M", // sum offset
                "D=M",   // read D = RAM[THIS + offset]
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::This, 19),
        );
    }

    #[test]
    fn generate_push_that_15() {
        assert_instructions(
            &vec![
                "@15",   // load offset
                "D=A",   // store offset in D
                "@THAT", // THAT base addr
                "A=D+M", // sum offset
                "D=M",   // read D = RAM[THAT + offset]
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::That, 15),
        );
    }

    #[test]
    fn generate_push_pointer() {
        assert_instructions(
            &vec![
                "@THIS", // THIS base addr
                "D=M",   // read D = RAM[THIS]
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::Pointer, 0),
        );
        assert_instructions(
            &vec![
                "@THAT", // THIS base addr
                "D=M",   // read D = RAM[THIS]
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::Pointer, 1),
        );
    }

    #[test]
    fn generate_push_temp_4() {
        assert_instructions(
            &vec![
                "@9",  // TEMP for 4 (5 + 4)
                "D=M", // read D = RAM[9]
                "@SP", "A=M", "M=D", "@SP", "M=M+1",
            ],
            VMInstruction::Push(Segment::Temp, 4),
        );
    }

    #[test]
    fn generate_pop_local_2() {
        assert_instructions(
            &vec![
                "@2",    // load offset
                "D=A",   // store offset in D
                "@LCL",  // LOCAL base addr
                "D=D+M", // sum offset, store address in D
                "@R13", "M=D", // R13=D temporarly
                "@SP", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@R13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Local, 2),
        );
    }

    #[test]
    fn generate_pop_static_3() {
        assert_instructions(
            &vec![
                "@Test.3", // load offset
                "D=A",     // store address in D
                "@R13", "M=D", // R13=D temporarly
                "@SP", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@R13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Static, 3),
        );
    }

    #[test]
    fn generate_pop_this_10() {
        assert_instructions(
            &vec![
                "@10",   // load offset
                "D=A",   // store offset in D
                "@THIS", // THIS base addr
                "D=D+M", // sum offset, store address in D
                "@R13", "M=D", // R13=D temporarly
                "@SP", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@R13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::This, 10),
        );
    }

    #[test]
    fn generate_pop_pointer() {
        assert_instructions(
            &vec![
                "@THIS", // THIS base addr
                "D=A",   // store address in D TODO: OPTIMIZE THAT
                "@R13", "M=D", // R13=D temporarly
                "@SP", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@R13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Pointer, 0),
        );
        assert_instructions(
            &vec![
                "@THAT", // THAT base addr
                "D=A",   // store address in D TODO: OPTIMIZE THAT
                "@R13", "M=D", // R13=D temporarly
                "@SP", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@R13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Pointer, 1),
        );
    }

    #[test]
    fn generate_pop_that_44() {
        assert_instructions(
            &vec![
                "@44",   // load offset
                "D=A",   // store offset in D
                "@THAT", // THAT base addr
                "D=D+M", // sum offset, store address in D
                "@R13", "M=D", // R13=D temporarly
                "@SP", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@R13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::That, 44),
        );
    }

    #[test]
    fn generate_pop_temp_7() {
        assert_instructions(
            &vec![
                "@12", // TEMP (5 + 7)
                "D=A", // store address in D
                "@R13", "M=D", // R13=D temporarly
                "@SP", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@R13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Temp, 7),
        );
    }

    #[test]
    fn generate_pop_argument_9() {
        assert_instructions(
            &vec![
                "@9",    // load offset
                "D=A",   // store offset in D
                "@ARG",  // argument base addr
                "D=D+M", // sum offset, store address in D
                "@R13", "M=D", // R13=D temporarly
                "@SP", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@R13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Argument, 9),
        );
    }

    #[test]
    fn generate_add() {
        assert_instructions(
            &vec![
                //SP--, D=RAM[SP], RAM[SP-1]+=D
                "@SP", "M=M-1", "A=M", "D=M", "@SP", "A=M-1", "M=D+M",
            ],
            VMInstruction::Arithmetic(Arithmetic::Add),
        );
    }

    #[test]
    fn generate_sub() {
        assert_instructions(
            &vec![
                //SP--, D=RAM[SP], RAM[SP-1]-=D
                "@SP", "M=M-1", "A=M", "D=M", "@SP", "A=M-1", "M=M-D",
            ],
            VMInstruction::Arithmetic(Arithmetic::Sub),
        );
    }

    #[test]
    fn generate_neg() {
        assert_instructions(
            &vec![
                //SP--, D=RAM[SP], RAM[SP-1]-=D
                "@SP", "A=M-1", "M=-M",
            ],
            VMInstruction::Arithmetic(Arithmetic::Neg),
        );
    }

    #[test]
    fn generate_eq() {
        assert_instructions(
            &vec![
                "@SP", "M=M-1", "A=M", "D=M", "@SP", "A=M-1", "D=M-D", "@true.1", "D;JEQ", "D=0",
                "@end.2", "0;JMP", "(true.1)", "D=-1", "(end.2)", "@SP", "A=M-1", "M=D",
            ],
            VMInstruction::Arithmetic(Arithmetic::Eq),
        );
    }

    #[test]
    fn generate_gt() {
        assert_instructions(
            &vec![
                "@SP", "M=M-1", "A=M", "D=M", "@SP", "A=M-1", "D=M-D", "@true.1", "D;JGT", "D=0",
                "@end.2", "0;JMP", "(true.1)", "D=-1", "(end.2)", "@SP", "A=M-1", "M=D",
            ],
            VMInstruction::Arithmetic(Arithmetic::Gt),
        );
    }

    #[test]
    fn generate_lt() {
        assert_instructions(
            &vec![
                "@SP", "M=M-1", "A=M", "D=M", "@SP", "A=M-1", "D=M-D", "@true.1", "D;JLT", "D=0",
                "@end.2", "0;JMP", "(true.1)", "D=-1", "(end.2)", "@SP", "A=M-1", "M=D",
            ],
            VMInstruction::Arithmetic(Arithmetic::Lt),
        );
    }

    #[test]
    fn generate_and() {
        assert_instructions(
            &vec!["@SP", "M=M-1", "A=M", "D=M", "@SP", "A=M-1", "M=D&M"],
            VMInstruction::Arithmetic(Arithmetic::And),
        );
    }

    #[test]
    fn generate_or() {
        assert_instructions(
            &vec!["@SP", "M=M-1", "A=M", "D=M", "@SP", "A=M-1", "M=D|M"],
            VMInstruction::Arithmetic(Arithmetic::Or),
        );
    }

    #[test]
    fn generate_not() {
        assert_instructions(
            &vec![
                //SP--, D=RAM[SP], RAM[SP-1]-=D
                "@SP", "A=M-1", "M=!M",
            ],
            VMInstruction::Arithmetic(Arithmetic::Not),
        );
    }

    #[test]
    fn parse_label() {
        let mut parser = Parser::new();
        let instruction = parser.parse_line(&String::from("label ELSE"));
        assert_eq!(instruction, VMInstruction::Label(String::from("ELSE")));
    }

    #[test]
    fn parse_goto() {
        let mut parser = Parser::new();
        let instruction = parser.parse_line(&String::from("goto FAIL"));
        assert_eq!(instruction, VMInstruction::GoTo(String::from("FAIL")));
    }

    #[test]
    fn parse_if_goto() {
        let mut parser = Parser::new();
        let instruction = parser.parse_line(&String::from("if-goto FAIL"));
        assert_eq!(instruction, VMInstruction::IfGoTo(String::from("FAIL")));
    }

    #[test]
    fn parse_and_generate_function_call_with_return() {
        let mut parser = Parser::new();
        let return_instruction = parser.parse_line(&String::from("return"));
        assert_eq!(return_instruction, VMInstruction::Return);
        let function_instruction = parser.parse_line(&String::from("function Thing.foo 3"));
        assert_eq!(
            function_instruction,
            VMInstruction::Function("Thing.foo".to_string(), 3)
        );
        let call_instruction = parser.parse_line(&String::from("call AnotherThing.bar 2"));
        assert_eq!(
            call_instruction,
            VMInstruction::Call(VMFunctionCall {
                from: "Thing.foo".to_string(),
                target: "AnotherThing.bar".to_string(),
                arguments: 2
            })
        );
        assert_instructions(
            &vec![
                "(Thing.foo)", // file_name.function_name
                "@3",
                "D=A", // D = 3 (number of local variables)
                "@SP",
                "A=M",
                "M=0",
                "A=A+1",
                "M=0",
                "A=A+1",
                "M=0",
                "A=A+1",
                "@SP",
                "M=D+M", // push 0 D times
            ],
            function_instruction,
        );
        assert_instructions(
            &vec![
                "@Thing.foo$ret1", // file_name.function_name$ret<1..>
                "D=A",             // D = return label address
                "@SP",
                "M=M+1",
                "A=M-1",
                "M=D", // push D
                "@LCL",
                "D=M", // D=LCL
                "@SP",
                "M=M+1",
                "A=M-1",
                "M=D", // push D
                "@ARG",
                "D=M", // D=ARG
                "@SP",
                "M=M+1",
                "A=M-1",
                "M=D", // push ARG
                "@THIS",
                "D=M", // D=THIS
                "@SP",
                "M=M+1",
                "A=M-1",
                "M=D", // push THIS
                "@THAT",
                "D=M", // D=THAT
                "@SP",
                "M=M+1",
                "A=M-1",
                "M=D", // push THAT
                "@SP",
                "D=M",
                "@LCL",
                "M=D", // LCL = SP
                "@7",  // 5 + arguments = 7, at compile time
                "D=D-A",
                "@ARG",
                "M=D", // ARG = SP - 7
                "@AnotherThing.bar",
                "0;JMP",
                "(Thing.foo$ret1)", // return label
            ],
            call_instruction,
        );
        assert_instructions(
            &vec![
                "@LCL", "D=M", "@R13", "M=D", // R13 = LCL
                "@5", "D=A", "@R13", "D=M-D", "A=D", "D=M", "@R14", "M=D", // R14 = R13 - 5
                "@SP", "M=M-1", "A=M", "D=M", "@ARG", "A=M", "M=D", // *ARG = pop()
                "@ARG", "D=M+1", "@SP", "M=D", // SP = ARG+1
                "@R13", "AM=M-1", "D=M", "@THAT", "M=D", // THAT = *(--R13) == R13-1
                "@R13", "AM=M-1", "D=M", "@THIS", "M=D", // THIS = *(--R13) == R13-2
                "@R13", "AM=M-1", "D=M", "@ARG", "M=D", // ARG = *(--R13) == R13-3
                "@R13", "AM=M-1", "D=M", "@LCL", "M=D", // LCL = *(--R13) == R13-4
                "@R14", "A=M;JMP", // goto *R14
            ],
            return_instruction,
        );
    }

    #[test]
    fn generate_label_goto() {
        let mut parser = Parser::new();
        let label = parser.parse_line(&String::from("label Program.FAIL"));
        let conditional = parser.parse_line(&String::from("if-goto Program.FAIL"));
        let goto = parser.parse_line(&String::from("goto Program.FAIL"));
        assert_instructions(&vec!["(Program.FAIL)"], label);
        assert_instructions(
            &vec!["@SP", "M=M-1", "A=M", "D=M", "@Program.FAIL", "D;JNE"],
            conditional,
        );
        assert_instructions(&vec!["@Program.FAIL", "0;JMP"], goto);
    }
}
