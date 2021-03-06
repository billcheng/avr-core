use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct Bclr {
  pub(in crate::avr) s: usize,
}

impl Bclr {
  pub fn new(opcode: u16) -> Self {
    let s = (opcode & 0b0000_0000_0111_0000) >> 4;

    Self { s: s as usize }
  }
}

impl Instruction for Bclr {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set(self.s, false);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn bclr_0_clears_carry() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Bclr::new(0b1001_0100_1000_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn bclr_1_clears_zero() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_zero(true);
    }

    let op = super::Bclr::new(0b1001_0100_1001_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn bclr_2_clears_negative() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_negative(true);
    }

    let op = super::Bclr::new(0b1001_0100_1010_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_negative(), 0);
  }

  #[test]
  fn bclr_3_clears_overflow() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_overflow(true);
    }

    let op = super::Bclr::new(0b1001_0100_1011_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn bclr_4_clears_sign() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_sign(true);
    }

    let op = super::Bclr::new(0b1001_0100_1100_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn bclr_5_clears_half_carry() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_half_carry(true);
    }

    let op = super::Bclr::new(0b1001_0100_1101_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_half_carry(), 0);
  }

  #[test]
  fn bclr_6_clears_transfer() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(true);
    }

    let op = super::Bclr::new(0b1001_0100_1110_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_transfer(), 0);
  }

  #[test]
  fn bclr_7_clears_interrupt() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_interrupt(true);
    }

    let op = super::Bclr::new(0b1001_0100_1111_1000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_interrupt(), 0);
  }
}
