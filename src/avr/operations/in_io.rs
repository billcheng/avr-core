use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct In {
  d: usize,
  a: usize,
}

impl In {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let a = ((opcode & 0b0000_0000_0000_1111) | ((opcode & 0b0000_0110_0000_0000) >> 5)) as usize;
    Self { d, a }
  }
}

impl Operation for In {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let io = execution_data.io.borrow();
    let mut registers = execution_data.registers.borrow_mut();
    registers.set(self.d, io.get(self.a));

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn in_rd0x17_io0x2f_returns_0x55() {
    let (registers_ptr, status_register_ptr, data_memory, io_ptr) = init(vec![]);
    {
      let mut io = io_ptr.borrow_mut();
      io.set(0x2f, 0x55);
    }

    let op = super::In::new(0b1011_0101_0111_1111);
    op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0000,
      data_memory: data_memory,
      io: io_ptr,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(0x17), 0x55);
  }
}
