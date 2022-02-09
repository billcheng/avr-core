use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Styq {
  r: usize,
  q: usize,
}

impl Styq {
  pub fn new(opcode: u16) -> Self {
    let r = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let q = (opcode & 0b0000_0000_0000_0111
      | ((opcode & 0b0000_1100_0000_0000) >> 7)
      | ((opcode & 0b0010_0000_0000_0000) >> 8)) as usize;

    Self { r, q }
  }
}

impl Instruction for Styq {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();
    let y = registers.get_y();
    let rr = registers.get(self.r);

    let mut data_memory = execution_data.data_memory.borrow_mut();
    data_memory.write((y as u64 + self.q as u64) as u32, rr);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn std_y50_q63_r31() {
    let testbed = init(vec![(31, 0x55)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_y(50);
    }

    let op = super::Styq::new(0b1010_1111_1111_1111);
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let data_memory = testbed.data_memory.borrow();
    assert_eq!(data_memory.read(50+63), 0x55);
  }
}
