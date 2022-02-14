use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Icall16 {}

impl Icall16 {
  pub fn new() -> Self {
    Self {}
  }
}

impl Instruction for Icall16 {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let stack_data = execution_data.pc + 1;

    let mut stack = execution_data.data_memory.borrow_mut();
    let mut registers = execution_data.registers.borrow_mut();
    stack.write(registers.get_stack_pointer(), (stack_data & 0xff) as u8);
    stack.write(
      (registers.get_stack_pointer() as i32 - 1) as u32,
      ((stack_data >> 8) & 0xff) as u8,
    );
    registers.add_stack_pointer(-2);

    (2, Some(registers.get_z() as u32)) // AVRe & AVRrc = 3, AVRxm & AVRxt = 2
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn icall_0x1234_changes_pc_to_0x1234() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(9);
      registers.set_z(0x1234);
    }

    let op = super::Icall16::new();
    let (_cycles, result) = op.execute(testbed);

    assert_eq!(result, Some(0x1234));
  }

  #[test]
  fn icall_0x1234_decrement_sp_by_2() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(9);
      registers.set_z(0x1234);
    }

    let op = super::Icall16::new();
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get_stack_pointer(), 7);
  }

  #[test]
  fn icall_0x1234_pushes_pc_2_bytes_to_stack() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(9);
      registers.set_z(0x1234);
    }

    let op = super::Icall16::new();
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      pc: 0x0123,
      ..testbed
    });

    let stack = testbed.data_memory.borrow();
    assert_eq!(stack.read(9), 0x24);
    assert_eq!(stack.read(8), 0x01);
  }
}
