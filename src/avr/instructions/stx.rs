use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Stx {
  pub(in crate::avr) r: usize,
}

impl Stx {
  pub fn new(opcode: u16) -> Self {
    let r = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { r }
  }
}

impl Instruction for Stx {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();
    let rr = registers.get(self.r);
    let x = registers.get_x();

    let mut data_memory = execution_data.data_memory.borrow_mut();
    data_memory.write(x as u32, rr);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use super::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn stx_0x00ff_r31() {
    let testbed = init(vec![(31, 0xaa)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_x(0x00ff);
    }

    let op = super::Stx::new(0b1001_0011_1111_1100);
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let data_memory = testbed.data_memory.borrow();
    assert_eq!(data_memory.read(0x00ff), 0xaa);
  }
}
