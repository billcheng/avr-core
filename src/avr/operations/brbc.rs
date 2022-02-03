use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Brbc {
  k: i8,
  s: usize,
}

impl Brbc {
  pub fn new(opcode: u16) -> Self {
    let k = (opcode & 0b0000_0011_1111_1000) >> 2;
    let s = opcode & 0b0000_0000_0000_0111;

    Self {
      k: k as i8 >> 1,
      s: s as usize,
    }
  }
}

impl Operation for Brbc {
  fn execute(
    &self,
    status_register: &mut StatusRegister,
    _registers: &mut Registers,
    pc: &u16,
  ) -> Option<u16> {
    let flag = status_register.get(self.s);
    let k = self.k as i8;

    match flag {
      0 => Some(((*pc as i32) + (k as i32)) as u16),
      _ => None,
    }
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;

  #[test]
  fn brbc_nc_0x0001_returns0x0000() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_carry(false);

    let op = super::Brbc::new(0b1111_0111_1111_1000);
    let result = op.execute(&mut status_register, &mut registers, &0x0001);

    assert_eq!(result, Some(0));
  }

  #[test]
  fn brbc_nc_0x0001_returns0x0002() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_carry(false);

    let op = super::Brbc::new(0b1111_0100_0000_1000);
    let result = op.execute(&mut status_register, &mut registers, &0x0001);

    assert_eq!(result, Some(2));
  }

  #[test]
  fn brbc_c_0x0001_returns_none() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_carry(true);

    let op = super::Brbc::new(0b1111_0100_0000_1000);
    let result = op.execute(&mut status_register, &mut registers, &0x0001);

    assert_eq!(result, None);
  }
}
