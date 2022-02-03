use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

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
  fn execute(&self, status_register: &mut StatusRegister, registers: &mut Registers, _pc: u32) -> Option<u32> {
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

  #[test]
  fn asr_0x80_returns_0b11000000() {
    let mut registers = super::Registers::new();
    registers.set(0, 0x80);
    let mut status_register = super::StatusRegister::new();

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(&mut status_register, &mut registers, 0x0000);

    assert_eq!(registers.get(0), 0b11000000);
  }

  #[test]
  fn asr_0x01_returns_zero() {
    let mut registers = super::Registers::new();
    registers.set(0, 0x00);
    let mut status_register = super::StatusRegister::new();

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(&mut status_register, &mut registers, 0x0000);

    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn asr_0x01_returns_carry() {
    let mut registers = super::Registers::new();
    registers.set(0, 0x01);
    let mut status_register = super::StatusRegister::new();

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(&mut status_register, &mut registers, 0x0000);

    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn asr_0x80_returns_negative() {
    let mut registers = super::Registers::new();
    registers.set(0, 0x80);
    let mut status_register = super::StatusRegister::new();

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(&mut status_register, &mut registers, 0x0000);

    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn asr_0x01_returns_overflow() {
    let mut registers = super::Registers::new();
    registers.set(0, 0x01);
    let mut status_register = super::StatusRegister::new();

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(&mut status_register, &mut registers, 0x0000);

    assert_eq!(status_register.get_overflow(), 1);
  }

  #[test]
  fn asr_0x01_returns_sign() {
    let mut registers = super::Registers::new();
    registers.set(0, 0x01);
    let mut status_register = super::StatusRegister::new();

    let and = super::Asr::new(0b1001_0100_0000_0101);
    and.execute(&mut status_register, &mut registers, 0x0000);

    assert_eq!(status_register.get_sign(), 1);
  }
}
