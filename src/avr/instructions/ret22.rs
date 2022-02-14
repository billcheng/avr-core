use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Ret22 {}

impl Ret22 {
  pub fn new() -> Self {
    Self {}
  }
}

impl Instruction for Ret22 {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let sp = registers.get_stack_pointer() as u64;

    let stack = execution_data.data_memory.borrow();
    let pc_22 = stack.read((sp + 1) as u32);
    let pc_high = stack.read((sp + 2) as u32);
    let pc_low = stack.read((sp + 3) as u32);

    let pc = ((pc_22 as u32) << 16) | ((pc_high as u32) << 8) | (pc_low as u32);

    registers.set_stack_pointer((sp + 3) as u32);

    (5, Some(pc)) // AVRe=5, AVRxm=5, AVRxt=5, AVRrc=NA
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn ret16() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(0xfc);

      let mut stack = testbed.data_memory.borrow_mut();
      stack.write(0xfd, 0x12);
      stack.write(0xfe, 0x34);
      stack.write(0xff, 0x56);
    }

    let op = super::Ret22::new();
    let (_cycles, result) = op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(result, Some(0x123456));
    assert_eq!(registers.get_stack_pointer(), 0xff);
  }
}
