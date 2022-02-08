use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Reti16 {}

impl Reti16 {
  pub fn new() -> Self {
    Self {}
  }
}

impl Instruction for Reti16 {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let sp = registers.get_stack_pointer() as u64;

    let stack = execution_data.data_memory.borrow();
    let pc_high = stack.read((sp + 1) as u32);
    let pc_low = stack.read((sp + 2) as u32);

    let pc = ((pc_high as u16) << 8) | (pc_low as u16);

    registers.set_stack_pointer((sp + 2) as u32);

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_interrupt(true);

    Some(pc as u32)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn reti16() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(0xfd);

      let mut stack = testbed.data_memory.borrow_mut();
      stack.write(0xfe, 0x12);
      stack.write(0xff, 0x34);
    }

    let op = super::Reti16::new();
    let result = op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(result, Some(0x1234));
    assert_eq!(registers.get_stack_pointer(), 0xff);
    assert_eq!(status_register.get_interrupt(), 1);
  }
}
