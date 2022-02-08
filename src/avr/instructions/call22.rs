use crate::avr::core_type::CoreType;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Call22 {
  k: u32,
}

impl Call22 {
  pub fn new(opcode: u16, next_opcode: u16) -> Self {
    let k1 = ((opcode & 0b0000_0001_1111_0000) >> 3) | opcode & 0b0000_0000_0000_0001;
    let k = ((k1 as u32) << 16) | (next_opcode as u32);

    Self {
      k,
    }
  }
}

impl Instruction for Call22 {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let stack_data = execution_data.pc + 2;

    let mut registers = execution_data.registers.borrow_mut();
    let mut stack = execution_data.data_memory.borrow_mut();
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
    Some(self.k)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::core_type::CoreType;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn call22_returns_stack_pointer_6() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(9);
    }

    let op = super::Call22::new(0b1001_010_1101_0_111_0, 0x5678);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      pc: 0x0001,
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get_stack_pointer(), 6);
  }

  #[test]
  fn call22_0x345678_returns_stack_data() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(9);
    }

    let op = super::Call22::new(0b1001_010_1101_0_111_0, 0x5678);
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      pc: 0x12345,
      ..testbed
    });

    let stack = testbed.data_memory.borrow();
    assert_eq!(stack.read(9), 0x47);
    assert_eq!(stack.read(8), 0x23);
    assert_eq!(stack.read(7), 0x01);
  }
}
