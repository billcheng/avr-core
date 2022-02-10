use crate::avr::instruction::InstructionData;
use crate::avr::instruction::Instruction;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct LdsAvrc {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) k: usize,
}

impl LdsAvrc {
  pub fn new(opcode: u16) -> Self {
    let d = 16 + ((opcode & 0b0000_0000_1111_0000) >> 4) as usize;
    let k = (opcode & 0x0f | (opcode & 0b0000_0111_0000_0000 >> 4)) as usize;

    Self { d, k }
  }
}

impl Instruction for LdsAvrc {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(self.k as u32);

    let mut registers = execution_data.registers.borrow_mut();
    registers.set(self.d, ds);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn ldsavrc_r31_0x7f() {
    let testbed = init(vec![]);
    {
      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(0x7f, 0xff)
    }

    let op = super::LdsAvrc::new(0b1010_0111_1111_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(31), 0xff);
  }
}
