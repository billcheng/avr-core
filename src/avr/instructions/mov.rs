use crate::avr::instruction::InstructionData;
use crate::avr::instruction::Instruction;

pub struct Mov {
  d: usize,
  r: usize,
}

impl Mov {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let r = (opcode & 0b0000_0000_0000_1111 | ((opcode & 0b0000_0010_0000_0000) >> 5)) as usize;

    Self { d, r }
  }
}

impl Instruction for Mov {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rr = registers.get(self.r);

    registers.set(self.d, rr);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn mov_r31_r30_return_0xff() {
    let testbed = init(vec![(31, 0xff)]);

    let op = super::Mov::new(0b0010_1111_1110_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(30), 0xff);
  }
}
