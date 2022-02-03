use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Andi {
  d: usize,
  k: u8,
}

impl Andi {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0000_1111_0000) >> 4;
    let k = (((opcode & 0b0000_1111_0000_0000) >> 4 | opcode & 0b0000_0000_0000_1111) & 0xff) as u8;

    Self { d: d as usize, k }
  }
}

impl Operation for Andi {
  fn execute(&self, status_register: &mut StatusRegister, registers: &mut Registers) {
    let rd = registers.get(self.d);
    let result = rd & self.k;
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
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;

  #[test]
  fn andi_r0_0x55_returns_0x00() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    let mut status_register = super::StatusRegister::new();

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(&mut status_register, &mut registers);

    assert_eq!(registers.get(0), 0x00);
  }

  #[test]
  fn andi_r0_0x55_returns_zero() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    let mut status_register = super::StatusRegister::new();

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(&mut status_register, &mut registers);

    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn andi_r0_0x55_returns_negative() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    let mut status_register = super::StatusRegister::new();

    let and = super::Andi::new(0b0111_1111_0000_1111);
    and.execute(&mut status_register, &mut registers);

    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn andi_r0_0x55_returns_overflow() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    let mut status_register = super::StatusRegister::new();

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(&mut status_register, &mut registers);

    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn andi_r0_0xff_returns_sign() {
    let mut registers = super::Registers::new();
    registers.set(0, 0xaa);
    let mut status_register = super::StatusRegister::new();

    let and = super::Andi::new(0b0111_1111_0000_1111);
    and.execute(&mut status_register, &mut registers);

    assert_eq!(status_register.get_sign(), 1);
  }
}
