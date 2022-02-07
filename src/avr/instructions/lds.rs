use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Lds {
  d: usize,
  k: u16,
}

impl Lds {
  pub fn new(opcode: u16, next_opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let k = next_opcode;

    Self { d, k }
  }
}

impl Instruction for Lds {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(self.k as u32);

    let mut registers = execution_data.registers.borrow_mut();
    registers.set(self.d, ds);

    Some(execution_data.pc + 2)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn lds_r31_0x0010() {
    let testbed = init(vec![]);
    {
      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(0x10, 0xff)
    }

    let op = super::Lds::new(0b1001_0001_1111_0000, 0x0010);
    let result = op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(31), 0xff);
    assert_eq!(result, Some(0x0002));
  }
}
