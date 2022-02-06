use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct LdsAvrc {
  d: usize,
  k: usize,
}

impl LdsAvrc {
  pub fn new(opcode: u16) -> Self {
    let d = 16 + ((opcode & 0b0000_0000_1111_0000) >> 4) as usize;
    let k = (opcode & 0x0f | (opcode & 0b0000_0111_0000_0000 >> 4)) as usize;

    Self { d, k }
  }
}

impl Operation for LdsAvrc {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(self.k as u32);

    let mut registers = execution_data.registers.borrow_mut();
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
  fn ldsavrc_r31_0x7f() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut mem = data_memory.borrow_mut();
      mem.write(0x7f, 0xff)
    }

    let op = super::LdsAvrc::new(0b1010_0111_1111_1111);
    op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0000,
      data_memory: data_memory,
      io: io,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(31), 0xff);
  }
}
