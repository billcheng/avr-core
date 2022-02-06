use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Fmulsu {
  d: usize,
  r: usize,
}

impl Fmulsu {
  pub fn new(opcode: u16) -> Self {
    let d = 16 + ((opcode & 0b0000_0000_0111_0000) >> 4) as usize;
    let r = 16 + (opcode & 0b0000_0000_0000_0111) as usize;

    Self { d, r }
  }
}

impl Operation for Fmulsu {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d) as i8;
    let rr = registers.get(self.r) as u8;
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

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn fmulsu_0x40_0x06_returns_0x0300() {
    let testbed = init(vec![(16, 0x40), (17, 0x06)]);

    let op = super::Fmulsu::new(0b0000_0011_1000_1001);
    op.execute(super::ExecutionData {
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
  fn fmulsu_0x40_0x00_returns_0x0000() {
    let testbed = init(vec![(16, 0x40), (17, 0x00)]);

    let op = super::Fmulsu::new(0b0000_0011_1000_1001);
    op.execute(super::ExecutionData {
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
  fn fmulsu_0xff_0xff_returns_0xfe02() {
    let testbed = init(vec![(16, 0xff), (17, 0xff)]);

    let op = super::Fmulsu::new(0b0000_0011_1000_1001);
    op.execute(super::ExecutionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x02);
    assert_eq!(registers.get(1), 0xfe);
    assert_eq!(status_register.get_carry(), 1);
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn fmulsu_0x7e_0xff_returns_0xfb04() {
    let testbed = init(vec![(16, 0x7e), (17, 0xff)]);

    let op = super::Fmulsu::new(0b0000_0011_1000_0001);
    op.execute(super::ExecutionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x04);
    assert_eq!(registers.get(1), 0xfb);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn fmulsu_0xff_0x7e_returns_0xff04() {
    let testbed = init(vec![(16, 0xff), (17, 0x7e)]);

    let op = super::Fmulsu::new(0b0000_0011_1000_0001);
    op.execute(super::ExecutionData {
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
