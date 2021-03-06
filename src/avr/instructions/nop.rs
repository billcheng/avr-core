use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Nop {
}

impl Nop {
  pub fn new() -> Self {

    Self { }
  }
}

impl Instruction for Nop {
  fn execute(&self, _execution_data: InstructionData) -> InstructionResult {
    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn nop_returns_none() {
    let testbed = init(vec![]);

    let op = super::Nop::new();
    let (_cycles, result) = op.execute(testbed);

    assert_eq!(result, None);
  }
}
