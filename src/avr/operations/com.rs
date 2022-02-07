use crate::avr::operation::Instruction;
use crate::avr::operation::InstructionData;

pub struct Com {
  d: usize,
}

impl Com {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Instruction for Com {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d);
    let result = 0xff - rd;
    registers.set(self.d, result);

    let r7 = (result >> 7 & 1) != 0;

    let overflow = false;
    let negative = r7;
    let zero = result == 0;
    let carry = true;
    let sign = negative ^ overflow;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_sign(sign);
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_carry(carry);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn com_r1_0x05_returns0xfa() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set(1, 0x05);
    }

    let op = super::Com::new(0b1001_0100_0001_0000);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(1), 0xfa);
  }

  #[test]
  fn com_r1_returns_nv() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set(1, 0x05);
    }

    let op = super::Com::new(0b1001_0100_0001_0000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn com_r1_returns_n_s_nz() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set(1, 0x05);
    }

    let op = super::Com::new(0b1001_0100_0001_0000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn com_r1_returns_nn_ns_nz() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set(1, 0xf0);
    }

    let op = super::Com::new(0b1001_0100_0001_0000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn com_r1_returns_z() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set(1, 0xff);
    }

    let op = super::Com::new(0b1001_0100_0001_0000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 1);
  }
}
