use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Or {
  pub(in crate::avr) r: usize,
  pub(in crate::avr) d: usize,
}

impl Or {
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

impl Instruction for Or {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d);
    let rr = registers.get(self.r);
    let result = rd | rr;

    let r7 = (result >> 7 & 1) != 0;

    let overflow = false;
    let negative = r7;
    let sign = negative ^ overflow;
    let zero = result == 0;

    registers.set(self.d, result);

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
  fn or_r30_r31_returns_0xff() {
    let testbed = init(vec![(30, 0x55), (31, 0xff)]);

    let op = super::Or::new(0b0010_1011_1110_1111);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0xff);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
  }

  #[test]
  fn or_r30_r31_returns_0x00() {
    let testbed = init(vec![(30, 0x00), (31, 0x00)]);

    let op = super::Or::new(0b0010_1011_1110_1111);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0x00);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn or_r30_r31_returns_0x7f() {
    let testbed = init(vec![(30, 0x70), (31, 0x0f)]);

    let op = super::Or::new(0b0010_1011_1110_1111);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0x7f);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

}
