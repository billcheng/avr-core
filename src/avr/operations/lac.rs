use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Lac {
  d: usize,
}

impl Lac {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Operation for Lac {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let rd = registers.get(self.d);
    let z = registers.get_z();

    let mut data_memory = execution_data.data_memory.borrow_mut();
    let ds = data_memory.read(z as u32);

    registers.set(self.d, ds);
    data_memory.write(z as u32, (0xff - rd) & ds);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn lac_rd_0xfe_mem_0xff_return_0xff_0x01() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(7, 0xfe)]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set_z(9);

      let mut mem = data_memory.borrow_mut();
      mem.write(9, 0xff)
    }

    let op = super::Lac::new(0b1001_0010_0111_0110);
    op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0000,
      data_memory: data_memory.clone(),
      io: io,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(7), 0xff);
    let mem = data_memory.borrow();
    assert_eq!(mem.read(9), 0x01);
  }
}
