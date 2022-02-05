use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct LdDec {
  d: usize,
}

impl LdDec {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Operation for LdDec {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let x = (registers.get_x() as i32 - 1) as u16;

    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(x as u32);

    registers.set(self.d, ds);
    registers.set_x(x);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn ldinc_r5_0x0008_returns_0xfe() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set_x(8);

      let mut mem = data_memory.borrow_mut();
      mem.write(7, 0xfe)
    }

    let op = super::LdDec::new(0b1001_0000_0101_1101);
    op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0000,
      data_memory: data_memory,
      io: io,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(5), 0xfe);
    assert_eq!(registers.get_x(), 7);
  }
}
