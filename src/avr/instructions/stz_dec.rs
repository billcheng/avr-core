use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct StzDec {
  pub(in crate::avr) r: usize,
}

impl StzDec {
  pub fn new(opcode: u16) -> Self {
    let r = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { r }
  }
}

impl Instruction for StzDec {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let rr = registers.get(self.r);
    let z = registers.get_z() as i32 - 1;

    let mut data_memory = execution_data.data_memory.borrow_mut();
    data_memory.write(z as u32, rr);
    registers.set_z(z as u16);

    (2, None) // AVRe=2, AVRxm=2, AVRxt=1, AVRrc=2
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use super::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn stzdec_0x00ff_r10() {
    let testbed = init(vec![(10, 0xaa)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(0x00ff);
    }

    let op = super::StzDec::new(0b1001_0010_1010_0010);
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let data_memory = testbed.data_memory.borrow();
    let registers = testbed.registers.borrow();
    assert_eq!(data_memory.read(0x00fe), 0xaa);
    assert_eq!(registers.get_z(), 0x00fe);
  }
}
