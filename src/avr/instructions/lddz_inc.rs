use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct LddzInc {
  pub(in crate::avr) d: usize,
}

impl LddzInc {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Instruction for LddzInc {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let z = registers.get_z();

    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(z as u32);

    registers.set(self.d, ds);
    registers.set_z((z as u32 + 1) as u16);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn lddzinc_r5_0x0007_returns_0xfe() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(7);

      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(7, 0xfe)
    }

    let op = super::LddzInc::new(0b1001_0000_0101_1001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(5), 0xfe);
    assert_eq!(registers.get_z(), 8);
  }
}
