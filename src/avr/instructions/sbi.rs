use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Sbi {
  pub(in crate::avr) a: usize,
  pub(in crate::avr) b: usize,
}

impl Sbi {
  pub fn new(opcode: u16) -> Self {
    let a = ((opcode & 0b0000_0000_1111_1000) >> 3) as usize;
    let b = (opcode & 0b0000_0000_0000_0111) as usize;

    Self { a, b }
  }
}

impl Instruction for Sbi {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut io = execution_data.io.borrow_mut();
    let io_a = io.get(self.a) | (1 << self.b);
    io.set(self.a, io_a);

    (1, None) // AVRe=2, AVRxm=1, AVRxt=1, AVRrc=1
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn sbi_31_7() {
    let testbed = init(vec![]);
    {
      let mut io = testbed.io.borrow_mut();
      io.set(31, 0x7f);
    }

    let op = super::Sbi::new(0b1001_1010_1111_1111);
    op.execute(super::InstructionData {
      io: testbed.io.clone(),
      ..testbed
    });

    let io = testbed.io.borrow();
    assert_eq!(io.get(31), 0xff);
  }
}
