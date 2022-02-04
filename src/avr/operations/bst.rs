use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

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
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();

    let rd = registers.get(self.d);
    let mask = 1 << self.b;

    let bit = rd & mask != 0;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_transfer(bit);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn bst_r0_returns_t() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0b0000_1000)]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_transfer(false);
    }

    let op = super::Bst::new(0b1111_1010_0000_0011);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0001,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_transfer(), 1);
  }

  #[test]
  fn bst_r0_returns_nt() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0b0000_1000)]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_transfer(false);
    }

    let op = super::Bst::new(0b1111_1010_0000_0111);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0001,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_transfer(), 0);
  }
}
