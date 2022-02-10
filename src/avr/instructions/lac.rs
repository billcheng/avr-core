use crate::avr::instruction::InstructionData;
use crate::avr::instruction::Instruction;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Lac {
  pub(in crate::avr) d: usize,
}

impl Lac {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Instruction for Lac {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
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
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn lac_rd_0xfe_mem_0xff_return_0xff_0x01() {
    let testbed = init(vec![(7, 0xfe)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(9);

      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(9, 0xff)
    }

    let op = super::Lac::new(0b1001_0010_0111_0110);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(7), 0xff);
    let mem = testbed.data_memory.borrow();
    assert_eq!(mem.read(9), 0x01);
  }
}
