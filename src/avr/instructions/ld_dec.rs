use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct LdDec {
  pub(in crate::avr) d: usize,
}

impl LdDec {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Instruction for LdDec {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let x = (registers.get_x() as i32 - 1) as u16;

    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(x as u32);

    registers.set(self.d, ds);
    registers.set_x(x);

    (2, None) // AVRxm = 3, AVRe = 2, AVRxt = 2, AVRrc 2/3
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn ldinc_r5_0x0008_returns_0xfe() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_x(8);

      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(7, 0xfe)
    }

    let op = super::LdDec::new(0b1001_0000_0101_1101);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(5), 0xfe);
    assert_eq!(registers.get_x(), 7);
  }
}
