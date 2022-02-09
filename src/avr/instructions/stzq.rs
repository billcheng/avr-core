use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Stzq {
  r: usize,
  q: usize,
}

impl Stzq {
  pub fn new(opcode: u16) -> Self {
    let r = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let q = (opcode & 0b0000_0000_0000_0111
      | ((opcode & 0b0000_1100_0000_0000) >> 7)
      | ((opcode & 0b0010_0000_0000_0000) >> 8)) as usize;

    Self { r, q }
  }
}

impl Instruction for Stzq {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();
    let z = registers.get_z();
    let rr = registers.get(self.r);

    let mut data_memory = execution_data.data_memory.borrow_mut();
    data_memory.write(z as u32 + self.q as u32, rr);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn std_z50_q63_r10() {
    let testbed = init(vec![(10, 0x55)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(50);
    }

    let op = super::Stzq::new(0b1010_1110_1010_0111);
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let data_memory = testbed.data_memory.borrow();
    assert_eq!(data_memory.read(50+63), 0x55);
  }
}
