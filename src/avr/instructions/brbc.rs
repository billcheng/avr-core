use crate::avr::instruction::InstructionData;
use crate::avr::instruction::Instruction;

pub struct Brbc {
  pub(in crate::avr) k: i8,
  pub(in crate::avr) s: usize,
}

impl Brbc {
  pub fn new(opcode: u16) -> Self {
    let k = (opcode & 0b0000_0011_1111_1000) >> 2;
    let s = opcode & 0b0000_0000_0000_0111;

    Self {
      k: k as i8 >> 1,
      s: s as usize,
    }
  }
}

impl Instruction for Brbc {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let status_register = execution_data.status_register.borrow();

    let flag = status_register.get(self.s);
    let k = self.k as i8;

    match flag {
      0 => Some(((execution_data.pc as i64) + (k as i64) + 1) as u32),
      _ => None,
    }
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn brbc_nc_0x0001_returns0x0001() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Brbc::new(0b1111_0111_1111_1000);
    let result = op.execute(super::InstructionData {
      pc: 0x0001,
      ..testbed
    });

    assert_eq!(result, Some(1));
  }

  #[test]
  fn brbc_nc_0x0001_returns0x0003() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Brbc::new(0b1111_0100_0000_1000);
    let result = op.execute(super::InstructionData {
      pc: 0x0001,
      ..testbed
    });

    assert_eq!(result, Some(3));
  }

  #[test]
  fn brbc_c_0x0001_returns_none() {
    let testbed = init(vec![]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Brbc::new(0b1111_0100_0000_1000);
    let result = op.execute(super::InstructionData {
      pc: 0x0001,
      ..testbed
    });

    assert_eq!(result, None);
  }
}
