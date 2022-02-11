use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Data {
  pub(in crate::avr) opcode: u16,
}

impl Data {
  pub fn new(opcode: u16) -> Self {
    Self { opcode }
  }
}

impl Instruction for Data {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    panic!("Data instruction not implemented");
  }
}
