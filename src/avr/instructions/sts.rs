use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Sts {
  r: usize,
  k: u16,
}

impl Sts {
  pub fn new(opcode: u16, next_opcode: u16) -> Self {
    let r = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let k = next_opcode;

    Self { r, k }
  }
}

impl Instruction for Sts {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();
    let rr = registers.get(self.r);

    let mut data_memory = execution_data.data_memory.borrow_mut();
    data_memory.write(self.k as u32, rr);

    Some(execution_data.pc + 2)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn sts_0x0010_r31() {
    let testbed = init(vec![(31, 0x55)]);

    let op = super::Sts::new(0b1001_0011_1111_0000, 0x0010);
    let result = op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let data_memory = testbed.data_memory.borrow();
    assert_eq!(data_memory.read(0x10), 0x55);
    assert_eq!(result, Some(0x0002));
  }
}
