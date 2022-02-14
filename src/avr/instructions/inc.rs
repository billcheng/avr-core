use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct Inc {
  pub(in crate::avr) d: usize,
}

impl Inc {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Instruction for Inc {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d) as i16;
    let result = rd + 1;

    registers.set(self.d, result as u8);

    let r7 = (result >> 7 & 1) != 0;

    let overflow = result & 0xff == 0b1000_0000;
    let negative = r7;
    let zero = result & 0xff == 0;
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
  fn inc_0xff_return_0x00() {
    let testbed = init(vec![(22, 0xff)]);

    let op = super::Inc::new(0b1001_0101_0110_1011);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(22), 0x00);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn inc_0x00_return_0x01() {
    let testbed = init(vec![(22, 0x00)]);

    let op = super::Inc::new(0b1001_0101_0110_1011);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(22), 0x01);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn inc_0x7f_return_0x80() {
    let testbed = init(vec![(22, 0x7f)]);

    let op = super::Inc::new(0b1001_0101_0110_1011);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(22), 0x80);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 1);
  }

  #[test]
  fn inc_0x80_return_0x81() {
    let testbed = init(vec![(22, 0x80)]);

    let op = super::Inc::new(0b1001_0101_0110_1011);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(22), 0x81);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
  }
}
