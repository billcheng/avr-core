use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Lsr {
  d: usize,
}

impl Lsr {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    Self { d }
  }
}

impl Instruction for Lsr {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let rd = registers.get(self.d);
    let result = rd >> 1;

    registers.set(self.d, result);

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_carry(rd & 0x01 != 0);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn lsr_r31_0xff_returns_0x7f() {
    let testbed = init(vec![(31, 0xff)]);

    let op = super::Lsr::new(0b1001_0101_1111_0110);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x7f);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn lsr_r31_0xfe_returns_0x7f() {
    let testbed = init(vec![(31, 0xfe)]);

    let op = super::Lsr::new(0b1001_0101_1111_0110);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x7f);
    assert_eq!(status_register.get_carry(), 0);
  }
}
