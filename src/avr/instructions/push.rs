use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Push {
  pub(in crate::avr) d: usize,
}

impl Push {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0001_1111_0000) >> 4;

    Self { d: d as usize }
  }
}

impl Instruction for Push {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let sp = registers.get_stack_pointer();

    let mut stack = execution_data.data_memory.borrow_mut();
    stack.write(sp, registers.get(self.d));

    registers.set_stack_pointer(((sp as i64) - 1) as u32);

    (1, None) // AVRe=2, AVRxm=1, AVRxt=1, AVRrc=1
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn push_r31() {
    let testbed = init(vec![(31, 0x88)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(0xff);
    }

    let op = super::Push::new(0b1001_0011_1111_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let data_memory = testbed.data_memory.borrow();
    assert_eq!(registers.get_stack_pointer(), 0xfe);
    assert_eq!(data_memory.read(0xff), 0x88);
  }
}
