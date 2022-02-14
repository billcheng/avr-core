use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::Instruction;

pub struct Jmp {
  pub(in crate::avr) k: u32,
}

impl Jmp {
  pub fn new(opcode: u16, next_opcode: u16) -> Self {
    let k1 = ((opcode & 0b0000_0001_1111_0000) >> 3) | opcode & 0b0000_0000_0000_0001;
    let k = ((k1 as u32) << 16) | (next_opcode as u32);

    Self {
      k,
    }
  }
}

impl Instruction for Jmp {
  fn execute(&self, _execution_data: InstructionData) -> InstructionResult {
    (3, Some(self.k))
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn jmp_0x345678_returns_0x345678() {
    let testbed = init(vec![]);

    let op = super::Jmp::new(0b1001_010_1101_0_110_0, 0x5678);
    let (_cycles, result) = op.execute(super::InstructionData {
      pc: 0x0001,
      ..testbed
    });

    assert_eq!(result, Some(0x345678));
  }
}
