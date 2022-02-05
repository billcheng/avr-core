use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Lddyq {
  d: usize,
  q: usize,
}

impl Lddyq {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let q = (opcode & 0b0000_0000_0000_0111
      | ((opcode & 0b0000_1100_0000_0000) >> 7)
      | ((opcode & 0b0010_0000_0000_0000) >> 8)) as usize;

    Self { d, q }
  }
}

impl Operation for Lddyq {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let y = registers.get_y();

    let data_memory = execution_data.data_memory.borrow_mut();
    let ds = data_memory.read(y as u32 + self.q as u32);

    registers.set(self.d, ds);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn lddyq_r5_0x0001_0x3f_returns_0xfe() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set_y(1);

      let mut mem = data_memory.borrow_mut();
      mem.write(0x40, 0xfe)
    }

    let op = super::Lddyq::new(0b1010_1100_0101_1111);
    op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0000,
      data_memory: data_memory,
      io: io,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(5), 0xfe);
  }
}
