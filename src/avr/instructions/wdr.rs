use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Wdr {
}

impl Wdr {
  pub fn new() -> Self {

    Self { }
  }
}

impl Instruction for Wdr {
  fn execute(&self, _execution_data: InstructionData) -> InstructionResult {
    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn wdr_returns_none() {
    let testbed = init(vec![]);

    let op = super::Wdr::new();
    let (_cycles, result) = op.execute(testbed);

    assert_eq!(result, None);
  }
}
