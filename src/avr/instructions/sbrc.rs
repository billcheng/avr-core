use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::util::opcode_size::Opcode;
use crate::avr::util::opcode_size::OpcodeSize;

pub struct Sbrc {
  pub(in crate::avr) r: usize,
  pub(in crate::avr) b: usize,
  pub(in crate::avr) opcode_size: usize,
}

impl Sbrc {
  pub fn new(opcode: u16, next_opcode: u16, opcode_util: &Opcode) -> Self {
    let r = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let b = (opcode & 0b0000_0000_0000_0111) as usize;

    Self {
      r,
      b,
      opcode_size: opcode_util.get_size(next_opcode),
    }
  }
}

impl Instruction for Sbrc {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let registers = execution_data.registers.borrow();
    let rr = registers.get(self.r);

    match rr & (1 << self.b) == 0 {
      true => (2, Some((execution_data.pc as u64 + 1 + self.opcode_size as u64) as u32)),
      false => (1, None),
    }
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use super::Opcode;
  use crate::avr::test::test_init::init;

  #[test]
  fn sbrc_31_7_doesnt_skip() {
    let testbed = init(vec![(31, 0xff)]);

    let op = super::Sbrc::new(0b1111_1101_1111_0111, 0x00, &Opcode::new());
    let (_cycles, result) = op.execute(testbed);

    assert_eq!(result, None);
  }

  #[test]
  fn sbrc_31_7_skip_1_word() {
    let testbed = init(vec![(31, 0x7f)]);

    let op = super::Sbrc::new(0b1111_1101_1111_0111, 0x00, &Opcode::new());
    let (_cycles, result) = op.execute(super::InstructionData {
      pc: 0x0005,
      ..testbed
    });

    assert_eq!(result, Some(0x0007));
  }

  #[test]
  fn sbrc_31_7_skip_2_word() {
    let testbed = init(vec![(31, 0x7f)]);

    let op = super::Sbrc::new(0b1111_1101_1111_0111, 0b1001_0100_0000_1110, &Opcode::new());
    let (_cycles, result) = op.execute(super::InstructionData {
      pc: 0x0005,
      ..testbed
    });

    assert_eq!(result, Some(0x0008));
  }
}
