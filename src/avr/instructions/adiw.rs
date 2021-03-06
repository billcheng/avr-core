use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct Adiw {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) k: u8,
}

impl Adiw {
  pub fn new(opcode: u16) -> Self {
    let d = 24 + ((opcode & 0b0000_0000_0011_0000) >> 3);
    let decoded_k1 = opcode & 0b0000_0000_0000_1111;
    let decoded_k2 = (opcode & 0b0000_0000_1100_0000) >> 2;
    let k = decoded_k1 | decoded_k2;

    Self {
      d: d as usize,
      k: k as u8,
    }
  }
}

impl Instruction for Adiw {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d) as u16 | ((registers.get(self.d + 1) as u16) << 8);
    let result = rd as u32 + self.k as u32;
    registers.set(self.d, (result & 0x00ff) as u8);
    registers.set(self.d + 1, ((result & 0xff00) >> 8) as u8);

    let rdh7 = (rd >> 15 & 1) != 0;
    let r15 = (result >> 15 & 1) != 0;

    let overflow = !rdh7 & r15;
    let negative = r15;
    let zero = result & 0xffff == 0;
    let carry = rdh7 & !r15;
    let sign = negative ^ overflow;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_carry(carry);
    status_register.set_sign(sign);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn adiw_r24_63_returns0x023e_with_status_registers() {
    let testbed = init(vec![(24, 0xff), (25, 0x01)]);

    let adiw = super::Adiw::new(0x96cf);
    adiw.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(24), 0x3e);
    assert_eq!(registers.get(25), 0x02);
  }

  #[test]
  fn adiw_r24_0x01_returns0x0200_with_status_registers() {
    let testbed = init(vec![(24, 0xff), (25, 0x01)]);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(24), 0x00);
    assert_eq!(registers.get(25), 0x02);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn adiw_r24_0x01_returns_carry() {
    let testbed = init(vec![(24, 0xff), (25, 0xff)]);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn adiw_r24_0x01_returns_zero() {
    let testbed = init(vec![(24, 0xff), (25, 0xff)]);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn adiw_r24_0x01_returns_negative() {
    let testbed = init(vec![(24, 0xff), (25, 0xef)]);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn adiw_r24_0x01_returns_overflow() {
    let testbed = init(vec![(24, 0xff), (25, 0x7f)]);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_overflow(), 1);
  }

  #[test]
  fn adiw_r24_0x01_returns_sign() {
    let testbed = init(vec![(24, 0xff), (25, 0xef)]);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_sign(), 1);
  }
}
