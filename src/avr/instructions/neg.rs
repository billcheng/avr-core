use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Neg {
  pub(in crate::avr) d: usize,
}

impl Neg {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Instruction for Neg {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d) as i16;
    let result = (((-rd) as u16) & 0xff) as u8;

    let r3 = result & 0b0000_1000 != 0;
    let rd3 = result & 0b0000_1000 != 0;
    let r7 = result & 0b1000_0000 != 0;

    let half_carry = r3 | rd3;
    let overflow = result == 0b1000_0000;
    let negative = r7;
    let zero = result == 0x00;
    let carry = result != 0x00;
    let sign = negative ^ overflow;

    registers.set(self.d, result);

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_half_carry(half_carry);
    status_register.set_sign(sign);
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_carry(carry);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn neg_r31_returns_0xab() {
    let testbed = init(vec![(31, 0x55)]);

    let op = super::Neg::new(0b1001_0101_1111_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0xab);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn neg_r31_returns_0x00() {
    let testbed = init(vec![(31, 0x00)]);

    let op = super::Neg::new(0b1001_0101_1111_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x00);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_carry(), 0 );
  }

  #[test]
  fn neg_r31_returns_0x01() {
    let testbed = init(vec![(31, 0xff)]);

    let op = super::Neg::new(0b1001_0101_1111_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x01);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_carry(), 1);
  }
}
