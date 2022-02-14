use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct In {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) a: usize,
}

impl In {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let a = ((opcode & 0b0000_0000_0000_1111) | ((opcode & 0b0000_0110_0000_0000) >> 5)) as usize;
    Self { d, a }
  }
}

impl Instruction for In {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let io = execution_data.io.borrow();
    let mut registers = execution_data.registers.borrow_mut();
    registers.set(self.d, io.get(self.a));

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn in_rd0x17_io0x2f_returns_0x55() {
    let testbed = init(vec![]);
    {
      let mut io = testbed.io.borrow_mut();
      io.set(0x2f, 0x55);
    }

    let op = super::In::new(0b1011_0101_0111_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0x17), 0x55);
  }
}
