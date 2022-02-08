use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Ror {
  d: usize,
}

impl Ror {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0001_1111_0000) >> 4;

    Self { d: d as usize }
  }
}

impl Instruction for Ror {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let rd = registers.get(self.d);

    let mut status_register = execution_data.status_register.borrow_mut();
    let carry = status_register.get_carry();

    let result = (rd >> 1) | (carry << 7);
    let new_carry = rd & 0x01 != 0;

    registers.set(self.d, result);
    status_register.set_carry(new_carry);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn ror_r31_0xff_nc() {
    let testbed = init(vec![(31,0xff)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Ror::new(0b1001_0101_1111_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x7f);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn ror_r31_0x7f_nc() {
    let testbed = init(vec![(31,0x7f)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Ror::new(0b1001_0101_1111_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x3f);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn ror_r31_0x7e_nc() {
    let testbed = init(vec![(31,0x7e)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Ror::new(0b1001_0101_1111_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x3f);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn ror_r31_0xff_c() {
    let testbed = init(vec![(31,0xff)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Ror::new(0b1001_0101_1111_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0xff);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn ror_r31_0x7f_c() {
    let testbed = init(vec![(31,0x7f)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Ror::new(0b1001_0101_1111_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0xbf);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn ror_r31_0x7e_c() {
    let testbed = init(vec![(31,0x7e)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Ror::new(0b1001_0101_1111_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0xbf);
    assert_eq!(status_register.get_carry(), 0);
  }
}
