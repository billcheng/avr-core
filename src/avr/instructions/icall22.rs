use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Icall22 {}

impl Icall22 {
  pub fn new() -> Self {
    Self {}
  }
}

impl Instruction for Icall22 {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let stack_data = execution_data.pc + 1;

    let mut stack = execution_data.data_memory.borrow_mut();
    let mut registers = execution_data.registers.borrow_mut();
    stack.write(registers.get_stack_pointer(), (stack_data & 0xff) as u8);
    stack.write(
      (registers.get_stack_pointer() as i32 - 1) as u32,
      ((stack_data >> 8) & 0xff) as u8,
    );
    stack.write(
      (registers.get_stack_pointer() as i32 - 2) as u32,
      ((stack_data >> 16) & 0xff) as u8,
    );

    registers.add_stack_pointer(-3);

    Some(registers.get_z() as u32)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn icall_0x1234_decrement_sp_by_3() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(9);
      registers.set_z(0x1234);
    }

    let op = super::Icall22::new();
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get_stack_pointer(), 6);
  }

  #[test]
  fn icall_0x1234_pushes_pc_3_bytes_to_stack() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(9);
      registers.set_z(0x1234);
    }

    let op = super::Icall22::new();
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      pc: 0x123456,
      ..testbed
    });

    let stack = testbed.data_memory.borrow();
    assert_eq!(stack.read(9), 0x57);
    assert_eq!(stack.read(8), 0x34);
    assert_eq!(stack.read(7), 0x12);
  }
}
