use crate::avr::operation::InstructionData;
use crate::avr::operation::Instruction;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Lddzq {
  d: usize,
  q: usize,
}

impl Lddzq {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let q = (opcode & 0b0000_0000_0000_0111
      | ((opcode & 0b0000_1100_0000_0000) >> 7)
      | ((opcode & 0b0010_0000_0000_0000) >> 8)) as usize;

    Self { d, q }
  }
}

impl Instruction for Lddzq {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let z = registers.get_z();

    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(z as u32 + self.q as u32);

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
  fn lddzq_r5_0x0001_0x3f_returns_0xfe() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(1);

      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(0x40, 0xfe)
    }

    let op = super::Lddzq::new(0b1010_1100_0101_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(5), 0xfe);
  }
}
