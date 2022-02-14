use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct Data {
  pub(in crate::avr) opcode: u16,
}

impl Data {
  pub fn new(opcode: u16) -> Self {
    Self { opcode }
  }
}

impl Instruction for Data {
  fn execute(&self, _execution_data: InstructionData) -> InstructionResult {
    panic!("Data instruction not implemented");
  }
}
