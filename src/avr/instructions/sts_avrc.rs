use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct StsAvrc {
  r: usize,
  k: usize,
}

impl StsAvrc {
  pub fn new(opcode: u16) -> Self {
    let r = 16 + ((opcode & 0b0000_0000_1111_0000) >> 4) as usize;
    let k = (opcode & 0x0f | (opcode & 0b0000_0111_0000_0000 >> 4)) as usize;

    Self { r, k }
  }
}

impl Instruction for StsAvrc {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();
    let rr = registers.get(self.r);

    let mut data_memory = execution_data.data_memory.borrow_mut();
    data_memory.write(self.k as u32, rr);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn stsavrc_0x7f_r31() {
    let testbed = init(vec![(31, 0xff)]);

    let op = super::StsAvrc::new(0b1010_1111_1111_1111);
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let data_memory = testbed.data_memory.borrow();
    assert_eq!(data_memory.read(0x7f), 0xff);
  }
}
