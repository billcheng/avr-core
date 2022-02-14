use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::Instruction;

pub struct Fmuls {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) r: usize,
}

impl Fmuls {
  pub fn new(opcode: u16) -> Self {
    let d = 16 + ((opcode & 0b0000_0000_0111_0000) >> 4) as usize;
    let r = 16 + (opcode & 0b0000_0000_0000_0111) as usize;

    Self { d, r }
  }
}

impl Instruction for Fmuls {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d) as i8;
    let rr = registers.get(self.r) as i8;
    let result = rd as i16 * rr as i16;
    let after_shifting = result << 1;

    registers.set(0, (after_shifting & 0xff) as u8);
    registers.set(1, (after_shifting >> 8 & 0xff) as u8);

    let r16 = (result >> 15 & 1) != 0;

    let carry = r16;
    let zero = after_shifting == 0;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_carry(carry);
    status_register.set_zero(zero);

    (2, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn fmuls_0x40_0x06_returns_0x0300() {
    let testbed = init(vec![(16, 0x40), (17, 0x06)]);

    let op = super::Fmuls::new(0b0000_0011_1000_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x00);
    assert_eq!(registers.get(1), 0x03);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn fmuls_0x40_0x00_returns_0x0000() {
    let testbed = init(vec![(16, 0x40), (17, 0x00)]);

    let op = super::Fmuls::new(0b0000_0011_1000_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x00);
    assert_eq!(registers.get(1), 0x00);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn fmuls_0xff_0xff_returns_0x0002() {
    let testbed = init(vec![(16, 0xff), (17, 0xff)]);

    let op = super::Fmuls::new(0b0000_0011_1000_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x02);
    assert_eq!(registers.get(1), 0x00);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn fmuls_0x7e_0xff_returns_0xff04() {
    let testbed = init(vec![(16, 0x7e), (17, 0xff)]);

    let op = super::Fmuls::new(0b0000_0011_1000_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x04);
    assert_eq!(registers.get(1), 0xff);
    assert_eq!(status_register.get_carry(), 1);
    assert_eq!(status_register.get_zero(), 0);
  }
}
