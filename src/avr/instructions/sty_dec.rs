use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct StyDec {
  r: usize,
}

impl StyDec {
  pub fn new(opcode: u16) -> Self {
    let r = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { r }
  }
}

impl Instruction for StyDec {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let rr = registers.get(self.r);
    let y = registers.get_y() as i32 - 1;

    let mut data_memory = execution_data.data_memory.borrow_mut();
    data_memory.write(y as u32, rr);
    registers.set_y(y as u16);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use super::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn stydec_0x00ff_r31() {
    let testbed = init(vec![(31, 0xaa)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_y(0x00ff);
    }

    let op = super::StyDec::new(0b1001_0011_1111_1110);
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let data_memory = testbed.data_memory.borrow();
    let registers = testbed.registers.borrow();
    assert_eq!(data_memory.read(0x00fe), 0xaa);
    assert_eq!(registers.get_y(), 0x00fe);
  }
}
