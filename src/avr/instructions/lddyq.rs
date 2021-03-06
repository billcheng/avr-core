use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::Instruction;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Lddyq {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) q: usize,
}

impl Lddyq {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let q = (opcode & 0b0000_0000_0000_0111
      | ((opcode & 0b0000_1100_0000_0000) >> 7)
      | ((opcode & 0b0010_0000_0000_0000) >> 8)) as usize;

    Self { d, q }
  }
}

impl Instruction for Lddyq {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let y = registers.get_y();

    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(y as u32 + self.q as u32);

    registers.set(self.d, ds);

    (2, None) // AVRrc = NA, AVRe = 2, AVRxm = 3, AVRxt = 2
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn lddyq_r5_0x0001_0x3f_returns_0xfe() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_y(1);

      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(0x40, 0xfe)
    }

    let op = super::Lddyq::new(0b1010_1100_0101_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(5), 0xfe);
  }
}
