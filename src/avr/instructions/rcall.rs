use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;
use crate::CoreType;

pub struct Rcall {
  k: i16,
  core_type: CoreType,
}

impl Rcall {
  pub fn new(core_type: &CoreType, opcode: u16) -> Self {
    let k = (opcode & 0b0000_1111_1111_1111
      | match opcode & 0b0000_1000_0000_0000 {
        0b0000_0000_0000_0000 => 0,
        _ => 0b1111_0000_0000_0000,
      }) as i16;

    Self {
      k,
      core_type: core_type.clone(),
    }
  }
}

impl Instruction for Rcall {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let pc = execution_data.pc;
    let next_pc = pc as i64 + 1;
    let result = next_pc + self.k as i64;

    let mut registers = execution_data.registers.borrow_mut();
    let sp = registers.get_stack_pointer() as i64;

    let mut stack = execution_data.data_memory.borrow_mut();
    stack.write(sp as u32, ((next_pc as u32) & 0xff) as u8);
    stack.write((sp - 1) as u32, ((next_pc as u32 >> 8) & 0xff) as u8);
    if self.core_type == CoreType::Bits22 {
      stack.write((sp - 2) as u32, ((next_pc as u32 >> 16) & 0xff) as u8);
    }

    registers.set_stack_pointer(
      (sp
        - match self.core_type {
          CoreType::Bits16 => 2,
          CoreType::Bits22 => 3,
        }) as u32,
    );

    Some(result as u32)
  }
}

#[cfg(test)]
mod test {
  use super::CoreType;
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn rcall_bits16_1000() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(0xff);
    }

    let op = super::Rcall::new(&CoreType::Bits16, 0b1101_0011_1110_1000);
    let result = op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      pc: 1500,
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let stack = testbed.data_memory.borrow();
    let registers = testbed.registers.borrow();
    assert_eq!(result, Some(2501));
    assert_eq!(stack.read(0xff), 0xdd);
    assert_eq!(stack.read(0xfe), 0x05);
    assert_eq!(registers.get_stack_pointer(), 0xfd);
  }

  #[test]
  fn rcall_bits22_1000() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(0xff);
    }

    let op = super::Rcall::new(&CoreType::Bits22, 0b1101_0011_1110_1000);
    let result = op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      pc: 0x123456,
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let stack = testbed.data_memory.borrow();
    let registers = testbed.registers.borrow();
    assert_eq!(result, Some(0x12383f));
    assert_eq!(stack.read(0xff), 0x57);
    assert_eq!(stack.read(0xfe), 0x34);
    assert_eq!(stack.read(0xfd), 0x12);
    assert_eq!(registers.get_stack_pointer(), 0xfc);
  }
}
