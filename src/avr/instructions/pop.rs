use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Pop {
  pub(in crate::avr) d: usize,
}

impl Pop {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0001_1111_0000) >> 4;

    Self { d: d as usize }
  }
}

impl Instruction for Pop {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let stack = execution_data.data_memory.borrow();
    let mut registers = execution_data.registers.borrow_mut();

    let sp = ((registers.get_stack_pointer() as u64) + 1) as u32;
    let result = stack.read(sp);

    registers.set(self.d, result);
    registers.set_stack_pointer(sp);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn pop_r31() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(0xfe);

      let mut data_memory = testbed.data_memory.borrow_mut();
      data_memory.write(0xff, 0x12);
    }

    let op = super::Pop::new(0b1001_0001_1111_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(31), 0x12);
    assert_eq!(registers.get_stack_pointer(), 0xff);
  }
}
