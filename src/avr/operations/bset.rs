use crate::avr::operation::InstructionData;
use crate::avr::operation::Instruction;

pub struct Bset {
  s: usize,
}

impl Bset {
  pub fn new(opcode: u16) -> Self {
    let s = (opcode & 0b0000_0000_0111_0000) >> 4;

    Self { s: s as usize }
  }
}

impl Instruction for Bset {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set(self.s, true);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn bset_nc_returns_c() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Bset::new(0b1001_0100_0000_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      pc: 0x0001,
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn bset_ni_returns_i() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_interrupt(false);
    }

    let op = super::Bset::new(0b1001_0100_0111_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      pc: 0x0001,
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_interrupt(), 1);
  }
}
