use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Sec {}

impl Sec {
  pub fn new() -> Self {
    Self {}
  }
}

impl Instruction for Sec {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut status_register = execution_data.status_register.borrow_mut();

    status_register.set_carry(true);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn sec() {
    let testbed = init(vec![]);

    let op = super::Sec::new();
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_carry(), 1);
  }
}