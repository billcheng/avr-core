use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Movw {
  d: usize,
  r: usize,
}

impl Movw {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0000_1111_0000) >> 3) as usize;
    let r = ((opcode & 0b0000_0000_0000_1111) << 1) as usize;

    Self { d, r }
  }
}

impl Instruction for Movw {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rr = registers.get(self.r);
    let rr_inc = registers.get(self.r + 1);

    registers.set(self.d, rr);
    registers.set(self.d + 1, rr_inc);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn mov_r31_r30_return_0xff() {
    let testbed = init(vec![(30, 0x55), (31, 0xaa)]);

    let op = super::Movw::new(0b0000_0001_1110_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(28), 0x55);
    assert_eq!(registers.get(29), 0xaa);
  }
}
