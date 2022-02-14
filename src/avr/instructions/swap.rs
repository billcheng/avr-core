use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Swap {
  pub(in crate::avr) d: usize,
}

impl Swap {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Instruction for Swap {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let rd = registers.get(self.d);

    let result = ((rd & 0x0f) << 4) | ((rd & 0xf0) >> 4);
    registers.set(self.d, result);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn swap_0xfe_returns_0xef() {
    let testbed = init(vec![(31, 0xfe)]);

    let op = super::Swap::new(0b1001_0101_1111_0010);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(31), 0xef);
  }

  #[test]
  fn swap_0xa5_returns_0x5a() {
    let testbed = init(vec![(31, 0xa5)]);

    let op = super::Swap::new(0b1001_0101_1111_0010);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(31), 0x5a );
  }
}
