use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Bst {
  d: usize,
  b: usize,
}

impl Bst {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0001_1111_0000) >> 4;
    let b = opcode & 0b0000_0000_0000_0111;

    Self {
      d: d as usize,
      b: b as usize,
    }
  }
}

impl Operation for Bst {
  fn execute(
    &self,
    status_register: &mut StatusRegister,
    registers: &mut Registers,
    _pc: u32,
    _data_memory: &DataMemoryPtr,
  ) -> Option<u32> {
    let rd = registers.get(self.d);
    let mask = 1 << self.b;

    let bit = rd & mask!=0;
    status_register.set_transfer(bit);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::data_memory::create_data_memory_ptr;
use crate::avr::operation::Operation;

  #[test]
  fn bst_r0_returns_t() {
    let mut registers = super::Registers::new();
    registers.set(0, 0b0000_1000);
    let mut status_register = super::StatusRegister::new();
    status_register.set_transfer(false);
    let data_memory = create_data_memory_ptr(10);

    let op = super::Bst::new(0b1111_1010_0000_0011);
    op.execute(&mut status_register, &mut registers, 0x0001, &data_memory);

    assert_eq!(status_register.get_transfer(), 1);
  }

  #[test]
  fn bst_r0_returns_nt() {
    let mut registers = super::Registers::new();
    registers.set(0, 0b0000_1000);
    let mut status_register = super::StatusRegister::new();
    status_register.set_transfer(false);
    let data_memory = create_data_memory_ptr(10);

    let op = super::Bst::new(0b1111_1010_0000_0111);
    op.execute(&mut status_register, &mut registers, 0x0001, &data_memory);

    assert_eq!(status_register.get_transfer(), 0);
  }
}
