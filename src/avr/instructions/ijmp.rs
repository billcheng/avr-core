use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Ijmp {}

impl Ijmp {
  pub fn new() -> Self {
    Self {}
  }
}

impl Instruction for Ijmp {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();

    Some(registers.get_z() as u32)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn ijmp_0x1234_changes_pc_to_0x1234() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(0x1234);
    }

    let op = super::Ijmp::new();
    let result = op.execute(testbed);

    assert_eq!(result, Some(0x1234));
  }
}
