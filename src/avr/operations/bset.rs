use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Bset {
  s: usize,
}

impl Bset {
  pub fn new(opcode: u16) -> Self {
    let s = (opcode & 0b0000_0000_0111_0000) >> 4;

    Self {
      s: s as usize,
    }
  }
}

impl Operation for Bset {
  fn execute(
    &self,
    status_register: &mut StatusRegister,
    _registers: &mut Registers,
    pc: &u16,
  ) -> Option<u16> {
    status_register.set(self.s, true);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;

  #[test]
  fn bset_nc_returns_c() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_carry(false);

    let op = super::Bset::new(0b1001_0100_0000_1000);
    op.execute(&mut status_register, &mut registers, &0x0001);

    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn bset_ni_returns_i() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_interrupt(false);

    let op = super::Bset::new(0b1001_0100_0111_1000);
    op.execute(&mut status_register, &mut registers, &0x0001);

    assert_eq!(status_register.get_interrupt(), 1);
  }
}
