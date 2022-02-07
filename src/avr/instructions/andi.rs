use crate::avr::instruction::InstructionData;
use crate::avr::instruction::Instruction;

pub struct Andi {
  d: usize,
  k: u8,
}

impl Andi {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0000_1111_0000) >> 4;
    let k = (((opcode & 0b0000_1111_0000_0000) >> 4 | opcode & 0b0000_0000_0000_1111) & 0xff) as u8;

    Self { d: d as usize, k }
  }
}

impl Instruction for Andi {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d);
    let result = rd & self.k;
    registers.set(self.d, result);

    let r7 = (result >> 7 & 1) != 0;

    let overflow = false;
    let negative = r7;
    let zero = result == 0;
    let sign = negative ^ overflow;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_sign(sign);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn andi_r0_0x55_returns_0x00() {
    let testbed = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0x00);
  }

  #[test]
  fn andi_r0_0x55_returns_zero() {
    let testbed = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn andi_r0_0x55_returns_negative() {
    let testbed = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_1111_0000_1111);
    and.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn andi_r0_0x55_returns_overflow() {
    let testbed = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn andi_r0_0xff_returns_sign() {
    let testbed = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_1111_0000_1111);
    and.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_sign(), 1);
  }
}
