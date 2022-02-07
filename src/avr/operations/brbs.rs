use crate::avr::operation::Instruction;
use crate::avr::operation::InstructionData;

pub struct Brbs {
  k: i8,
  s: usize,
}

impl Brbs {
  pub fn new(opcode: u16) -> Self {
    let k = (opcode & 0b0000_0011_1111_1000) >> 2;
    let s = opcode & 0b0000_0000_0000_0111;

    Self {
      k: k as i8 >> 1,
      s: s as usize,
    }
  }
}

impl Instruction for Brbs {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let status_register = execution_data.status_register.borrow();

    let flag = status_register.get(self.s);
    let k = self.k as i8;

    match flag {
      1 => Some(((execution_data.pc as i64) + (k as i64) + 1) as u32),
      _ => None,
    }
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn brbs_c_0x0001_returns0x0001() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Brbs::new(0b1111_0011_1111_1000);
    let result = op.execute(super::InstructionData {
      pc: 0x0001,
      ..testbed
    });

    assert_eq!(result, Some(1));
  }

  #[test]
  fn brbs_c_0x0001_returns0x0003() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Brbs::new(0b1111_0000_0000_1000);
    let result = op.execute(super::InstructionData {
      pc: 0x0001,
      ..testbed
    });

    assert_eq!(result, Some(3));
  }

  #[test]
  fn brbs_nc_0x0001_returns_none() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Brbs::new(0b1111_0000_0000_1000);
    let result = op.execute(super::InstructionData {
      pc: 0x0001,
      ..testbed
    });

    assert_eq!(result, None);
  }
}
