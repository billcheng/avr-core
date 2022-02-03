use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Brbs {
  k: i8,
  s: usize,
}

impl Brbs {
  pub fn new(opcode: u16) -> Self {
    let k = (opcode & 0b0000_0011_1111_1000) >> 2;
    let s = opcode & 0b0000_0000_0000_0111;

    Self {
      k: k as i8 >> 1,
      s: s as usize,
    }
  }
}

impl Operation for Brbs {
  fn execute(
    &self,
    status_register: &mut StatusRegister,
    _registers: &mut Registers,
    pc: u32,
    _data_memory: &DataMemoryPtr,
  ) -> Option<u32> {
    let flag = status_register.get(self.s);
    let k = self.k as i8;

    match flag {
      1 => Some(((pc as i64) + (k as i64) + 1) as u32),
      _ => None,
    }
  }
}

#[cfg(test)]
mod test {
  use crate::avr::data_memory::create_data_memory_ptr;
use crate::avr::operation::Operation;

  #[test]
  fn brbs_c_0x0001_returns0x0001() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_carry(true);
    let data_memory = create_data_memory_ptr(10);

    let op = super::Brbs::new(0b1111_0011_1111_1000);
    let result = op.execute(&mut status_register, &mut registers, 0x0001, &data_memory);

    assert_eq!(result, Some(1));
  }

  #[test]
  fn brbs_c_0x0001_returns0x0003() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_carry(true);
    let data_memory = create_data_memory_ptr(10);

    let op = super::Brbs::new(0b1111_0000_0000_1000);
    let result = op.execute(&mut status_register, &mut registers, 0x0001, &data_memory);

    assert_eq!(result, Some(3));
  }

  #[test]
  fn brbs_nc_0x0001_returns_none() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();
    status_register.set_carry(false);
    let data_memory = create_data_memory_ptr(10);

    let op = super::Brbs::new(0b1111_0000_0000_1000);
    let result = op.execute(&mut status_register, &mut registers, 0x0001, &data_memory);

    assert_eq!(result, None);
  }
}
