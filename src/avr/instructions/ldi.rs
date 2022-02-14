use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct Ldi {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) k: u8,
}

impl Ldi {
  pub fn new(opcode: u16) -> Self {
    let d = 16 + ((opcode & 0b0000_0000_1111_0000) >> 4);
    let k = (((opcode & 0b0000_1111_0000_0000) >> 4 | opcode & 0b0000_0000_0000_1111) & 0xff) as u8;

    Self { d: d as usize, k }
  }
}

impl Instruction for Ldi {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    registers.set(self.d, self.k);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn ldi_r26_0x55_returns_0x55() {
    let testbed = init(vec![]);

    let op = super::Ldi::new(0b1110_0101_1010_0101);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(26), 0x55);
  }
}
