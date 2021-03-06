use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct And {
  pub(in crate::avr) r: usize,
  pub(in crate::avr) d: usize,
}

impl And {
  pub fn new(opcode: u16) -> Self {
    let decoded_r1 = opcode & 0b0000_0000_0000_1111;
    let decoded_r2 = (opcode & 0b0000_0010_0000_0000) >> 5;
    let r = decoded_r1 | decoded_r2;

    let d = (opcode & 0b0000_0001_1111_0000) >> 4;

    Self {
      r: r as usize,
      d: d as usize,
    }
  }
}

impl Instruction for And {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d);
    let rr = registers.get(self.r);
    let result = rd & rr;
    registers.set(self.d, result);

    let r7 = (result >> 7 & 1) != 0;

    let overflow = false;
    let negative = r7;
    let zero = result == 0;
    let sign = negative ^ overflow;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_sign(sign);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn and_0xaa_0x55_returns_0x00() {
    let testbed = init(vec![(0, 0xaa), (1, 0x55)]);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0x00);
  }

  #[test]
  fn and_0xaa_0x55_returns_zero() {
    let testbed = init(vec![(0, 0xaa), (1, 0x55)]);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn and_0xaa_0xff_returns_negative() {
    let testbed = init(vec![(0, 0xaa), (1, 0xff)]);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn and_0xaa_0xff_returns_overflow() {
    let testbed = init(vec![(0, 0xaa), (1, 0xff)]);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn and_0xaa_0xff_returns_sign() {
    let testbed = init(vec![(0, 0xaa), (1, 0xff)]);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_sign(), 1);
  }
}
