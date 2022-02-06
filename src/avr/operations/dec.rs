use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Dec {
  d: usize,
}

impl Dec {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Operation for Dec {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d) as i16;
    let result = rd - 1;

    registers.set(self.d, result as u8);

    let r7 = (result >> 7 & 1) != 0;
    let r60 = result & 0b0111_1111 == 0b0111_1111;

    let overflow = !r7 & r60;
    let negative = r7;
    let zero = result == 0;
    let sign = negative ^ overflow;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_sign(sign);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn dec_0x01_return_0x00() {
    let testbed = init(vec![(22, 0x01)]);

    let op = super::Dec::new(0b1001_0101_0110_1010);
    op.execute(super::ExecutionData {
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
  fn dec_0x02_return_0x01() {
    let testbed = init(vec![(22, 0x02)]);

    let op = super::Dec::new(0b1001_0101_0110_1010);
    op.execute(super::ExecutionData {
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
  fn dec_0x00_return_0xff() {
    let testbed = init(vec![(22, 0x00)]);

    let op = super::Dec::new(0b1001_0101_0110_1010);
    op.execute(super::ExecutionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(22), 0xff);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn dec_0x80_return_0x7f() {
    let testbed = init(vec![(22, 0x80)]);

    let op = super::Dec::new(0b1001_0101_0110_1010);
    op.execute(super::ExecutionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(22), 0x7f);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 1);
  }
}
