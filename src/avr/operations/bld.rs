use crate::avr::operation::InstructionData;
use crate::avr::operation::Instruction;

pub struct Bld {
  d: usize,
  b: usize,
}

impl Bld {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0001_1111_0000) >> 4;
    let b = opcode & 0b0000_0000_0000_0111;

    Self {
      d: d as usize,
      b: b as usize,
    }
  }
}

impl Instruction for Bld {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let status_register = execution_data.status_register.borrow();

    let rd = registers.get(self.d);
    let t = status_register.get_transfer();

    let mask = 1 << self.b;

    let result = match t == 0 {
      true => rd & !mask,
      false => rd | mask,
    };

    registers.set(self.d, result);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn bld_t0_0xff_0_returns_0b11111110() {
    let testbed = init(vec![(0, 0xff)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(false);
    }

    let op = super::Bld::new(0b1111_1000_0000_0000);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0b1111_1110);
  }

  #[test]
  fn bld_t0_0xff_1_returns_0b11111101() {
    let testbed = init(vec![(0, 0xff)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(false);
    }

    let op = super::Bld::new(0b1111_1000_0000_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0b1111_1101);
  }

  #[test]
  fn bld_t0_0xff_2_returns_0b11111011() {
    let testbed = init(vec![(0, 0xff)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(false);
    }

    let op = super::Bld::new(0b1111_1000_0000_0010);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0b1111_1011);
  }

  #[test]
  fn bld_t0_0xff_3_returns_0b11110111() {
    let testbed = init(vec![(0, 0xff)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(false);
    }

    let op = super::Bld::new(0b1111_1000_0000_0011);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0b1111_0111);
  }

  #[test]
  fn bld_t0_0xff_7_returns_0b01111111() {
    let testbed = init(vec![(0, 0xff)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(false);
    }
    let op = super::Bld::new(0b1111_1000_0000_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0b0111_1111);
  }

  #[test]
  fn bld_t1_0x00_0_returns_0b00000001() {
    let testbed = init(vec![(0, 0x00)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(true);
    }

    let op = super::Bld::new(0b1111_1000_0000_0000);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0b0000_0001);
  }

  #[test]
  fn bld_t1_0x00_1_returns_0b00000010() {
    let testbed = init(vec![(0, 0x00)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(true);
    }

    let op = super::Bld::new(0b1111_1000_0000_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0b0000_0010);
  }

  #[test]
  fn bld_t1_0x00_7_returns_0b10000000() {
    let testbed = init(vec![(0, 0x00)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_transfer(true);
    }

    let op = super::Bld::new(0b1111_1000_0000_0111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0b1000_0000);
  }
}
