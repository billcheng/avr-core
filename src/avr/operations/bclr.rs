use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Bclr {
  s: usize,
}

impl Bclr {
  pub fn new(opcode: u16) -> Self {
    let s = (opcode & 0b0000_0000_0111_0000) >> 4;

    Self { s: s as usize }
  }
}

impl Operation for Bclr {
  fn execute(&self, status_register: &mut StatusRegister, _registers: &mut Registers, pc: &u16) -> Option<u16> {
    status_register.set(self.s, false);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;

  #[test]
  fn bclr_0_clears_carry() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_carry(true);

    let op = super::Bclr::new(0b1001_0100_1000_1000);
    op.execute(&mut status_register, &mut registers, &0x0000);

    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn bclr_1_clears_zero() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_zero(true);

    let op = super::Bclr::new(0b1001_0100_1001_1000);
    op.execute(&mut status_register, &mut registers, &0x0000);

    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn bclr_2_clears_negative() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_negative(true);

    let op = super::Bclr::new(0b1001_0100_1010_1000);
    op.execute(&mut status_register, &mut registers, &0x0000);

    assert_eq!(status_register.get_negative(), 0);
  }

  #[test]
  fn bclr_3_clears_overflow() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_overflow(true);

    let op = super::Bclr::new(0b1001_0100_1011_1000);
    op.execute(&mut status_register, &mut registers, &0x0000);

    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn bclr_4_clears_sign() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_sign(true);

    let op = super::Bclr::new(0b1001_0100_1100_1000);
    op.execute(&mut status_register, &mut registers, &0x0000);

    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn bclr_5_clears_half_carry() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_half_carry(true);

    let op = super::Bclr::new(0b1001_0100_1101_1000);
    op.execute(&mut status_register, &mut registers, &0x0000);

    assert_eq!(status_register.get_half_carry(), 0);
  }

  #[test]
  fn bclr_6_clears_transfer() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_transfer(true);

    let op = super::Bclr::new(0b1001_0100_1110_1000);
    op.execute(&mut status_register, &mut registers, &0x0000);

    assert_eq!(status_register.get_transfer(), 0);
  }

  #[test]
  fn bclr_7_clears_interrupt() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_interrupt(true);

    let op = super::Bclr::new(0b1001_0100_1111_1000);
    op.execute(&mut status_register, &mut registers, &0x0000);

    assert_eq!(status_register.get_interrupt(), 0);
  }
}
