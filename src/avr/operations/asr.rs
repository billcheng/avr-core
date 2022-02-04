use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Asr {
  d: usize,
}

impl Asr {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0001_1111_0000) >> 4;

    Self { d: d as usize }
  }
}

impl Operation for Asr {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let rd = registers.get(self.d);

    let result = rd >> 1 | (rd & 0x80);

    registers.set(self.d, result);

    let rd0 = (rd & 1) != 0;
    let r7 = (result >> 7 & 1) != 0;

    let carry = rd0;
    let negative = r7;
    let zero = result == 0;
    let overflow = negative ^ carry;
    let sign = negative ^ overflow;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_carry(carry);
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
  fn asr_0x80_returns_0b11000000() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0x80)]);

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(0), 0b11000000);
  }

  #[test]
  fn asr_0x01_returns_zero() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0x00)]);

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn asr_0x01_returns_carry() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0x01)]);

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn asr_0x80_returns_negative() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0x80)]);

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn asr_0x01_returns_overflow() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0x01)]);

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_overflow(), 1);
  }

  #[test]
  fn asr_0x01_returns_sign() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0x01)]);

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_sign(), 1);
  }
}
