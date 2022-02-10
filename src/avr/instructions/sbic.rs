use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::util::opcode_size::Opcode;
use crate::avr::util::opcode_size::OpcodeSize;

pub struct Sbic {
  pub(in crate::avr) a: usize,
  pub(in crate::avr) b: usize,
  pub(in crate::avr) opcode_size: usize,
}

impl Sbic {
  pub fn new(opcode: u16, next_opcode: u16, opcode_util: &Opcode) -> Self {
    let a = ((opcode & 0b0000_0000_1111_1000) >> 3) as usize;
    let b = (opcode & 0b0000_0000_0000_0111) as usize;

    Self {
      a,
      b,
      opcode_size: opcode_util.get_size(next_opcode),
    }
  }
}

impl Instruction for Sbic {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let io = execution_data.io.borrow_mut();
    let io_a = io.get(self.a);

    match io_a & (1 << self.b) == 0 {
      true => Some((execution_data.pc as u64 + 1 + self.opcode_size as u64) as u32),
      false => None,
    }
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use super::Opcode;
  use crate::avr::test::test_init::init;

  #[test]
  fn sbic_31_7_doesnt_skip() {
    let testbed = init(vec![]);
    {
      let mut io = testbed.io.borrow_mut();
      io.set(31, 0xff);
    }

    let op = super::Sbic::new(0b1001_1001_1111_1111, 0x00, &Opcode::new());
    let result = op.execute(testbed);

    assert_eq!(result, None);
  }

  #[test]
  fn sbic_31_7_skip_1_word() {
    let testbed = init(vec![]);
    {
      let mut io = testbed.io.borrow_mut();
      io.set(31, 0x7f);
    }

    let op = super::Sbic::new(0b1001_1001_1111_1111, 0x00, &Opcode::new());
    let result = op.execute(super::InstructionData {
      pc: 0x0005,
      ..testbed
    });

    assert_eq!(result, Some(0x0007));
  }

  #[test]
  fn sbic_31_7_skip_2_word() {
    let testbed = init(vec![]);
    {
      let mut io = testbed.io.borrow_mut();
      io.set(31, 0x7f);
    }

    let op = super::Sbic::new(0b1001_1001_1111_1111, 0b1001_0100_0000_1110, &Opcode::new());
    let result = op.execute(super::InstructionData {
      pc: 0x0005,
      ..testbed
    });

    assert_eq!(result, Some(0x0008));
  }
}
