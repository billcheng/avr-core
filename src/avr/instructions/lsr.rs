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

    let carry = rd & 0x01 != 0;
    let negative = false;
    let zero = result == 0;
    let overflow = negative ^ carry;
    let sign = negative ^ overflow;

    registers.set(self.d, result);

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_sign(sign);
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_carry(carry);

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
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 1);
    assert_eq!(status_register.get_sign(), 1);
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
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn lsr_r31_0x01_returns_0x00() {
    let testbed = init(vec![(31, 0x01)]);

    let op = super::Lsr::new(0b1001_0101_1111_0110);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x00);
    assert_eq!(status_register.get_carry(), 1);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 1);
    assert_eq!(status_register.get_sign(), 1);
  }
}
