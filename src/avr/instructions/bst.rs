use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct Bst {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) b: usize,
}

impl Bst {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0001_1111_0000) >> 4;
    let b = opcode & 0b0000_0000_0000_0111;

    Self {
      d: d as usize,
      b: b as usize,
    }
  }
}

impl Instruction for Bst {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let registers = execution_data.registers.borrow();

    let rd = registers.get(self.d);
    let mask = 1 << self.b;

    let bit = rd & mask != 0;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_transfer(bit);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn bst_r0_returns_t() {
    let testbed = init(vec![(0, 0b0000_1000)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(false);
    }

    let op = super::Bst::new(0b1111_1010_0000_0011);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      pc: 0x0001,
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_transfer(), 1);
  }

  #[test]
  fn bst_r0_returns_nt() {
    let testbed = init(vec![(0, 0b0000_1000)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(false);
    }

    let op = super::Bst::new(0b1111_1010_0000_0111);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      pc: 0x0001,
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_transfer(), 0);
  }
}
