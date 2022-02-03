use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct And {
  r: usize,
  d: usize,
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

impl Operation for And {
  fn execute(
    &self,
    status_register: &mut StatusRegister,
    registers: &mut Registers,
    _pc: u32,
    _data_memory: &DataMemoryPtr,
  ) -> Option<u32> {
    let rd = registers.get(self.d);
    let rr = registers.get(self.r);
    let result = rd & rr;
    registers.set(self.d, result);

    let r7 = (result >> 7 & 1) != 0;

    let overflow = false;
    let negative = r7;
    let zero = result == 0;
    let sign = negative ^ overflow;

    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_sign(sign);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::data_memory::create_data_memory_ptr;
  use crate::avr::operation::Operation;

  #[test]
  fn and_0xaa_0x55_returns_0x00() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    registers.set(1, 0x55);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(registers.get(0), 0x00);
  }

  #[test]
  fn and_0xaa_0x55_returns_zero() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    registers.set(1, 0x55);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn and_0xaa_0xff_returns_negative() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    registers.set(1, 0xff);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn and_0xaa_0xff_returns_overflow() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    registers.set(1, 0xff);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn and_0xaa_0xff_returns_sign() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    registers.set(1, 0xff);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let and = super::And::new(0b0010_0000_0000_0001);
    and.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_sign(), 1);
  }
}
