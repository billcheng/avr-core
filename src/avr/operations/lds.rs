use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Lds {
  d: usize,
  k: u16,
}

impl Lds {
  pub fn new(opcode: u16, next_opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let k = next_opcode;

    Self { d, k }
  }
}

impl Operation for Lds {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(self.k as u32);

    let mut registers = execution_data.registers.borrow_mut();
    registers.set(self.d, ds);

    Some(execution_data.pc + 2)
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn lds_r31_0x0010() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut mem = data_memory.borrow_mut();
      mem.write(0x10, 0xff)
    }

    let op = super::Lds::new(0b1001_0001_1111_0000, 0x0010);
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0000,
      data_memory: data_memory,
      io: io,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(31), 0xff);
    assert_eq!(result, Some(0x0002));
  }
}
