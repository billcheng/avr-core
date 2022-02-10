use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Adc {
  pub(in crate::avr) r: usize,
  pub(in crate::avr) d: usize,
}

impl Adc {
  pub fn new(opcode: u16) -> Self {
    let decoded_r1 = opcode & 0b0000_0000_0000_1111;
    let decoded_r2 = (opcode & 0b0000_0010_0000_0000) >> 5;
    let r = decoded_r1 | decoded_r2;

    let d = (opcode & 0b0000_0001_1111_0000) >> 4;

    Self {
      r: r as usize,
      d: d as usize,
    }
  }
}

impl Instruction for Adc {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let mut status_register = execution_data.status_register.borrow_mut();

    let rr = registers.get(self.r);
    let rd = registers.get(self.d);
    let result = rd as u16 + rr as u16 + status_register.get_carry() as u16;
    registers.set(self.d, result as u8);

    let rd3 = (rd >> 3 & 1) != 0;
    let rr3 = (rr >> 3 & 1) != 0;
    let r3 = (result >> 3 & 1) != 0;

    let rd7 = (rd >> 7 & 1) != 0;
    let rr7 = (rr >> 7 & 1) != 0;
    let r7 = (result >> 7 & 1) != 0;

    let half_carry = rd3 & rr3 | rr3 & !r3 | !r3 & rd3;
    let overflow = rd7 & rr7 & !r7 | !rd7 & !rr7 & r7;
    let negative = r7;
    let zero = result & 0xff == 0;
    let carry = rd7 & rr7 | rr7 & !r7 | !r7 & rd7;
    let sign = negative ^ overflow;

    status_register.set_half_carry(half_carry);
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_carry(carry);
    status_register.set_sign(sign);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn adc_0x01_x02_returns0x03_with_status_registers() {
    let testbed = init(vec![(0, 0x01), (1, 0x02)]);

    let adc = super::Adc::new(0b0001_1100_0000_0001);
    adc.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x03);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn adc_0x39_x48_returns0x81_with_status_registers() {
    let testbed = init(vec![(0, 0x39), (1, 0x48)]);

    let adc = super::Adc::new(0b0001_1100_0000_0001);
    adc.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x81);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_overflow(), 1);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn adc_0xff_xff_returns0xfe_with_status_registers() {
    let testbed = init(vec![(0, 0xff), (1, 0xff)]);

    let adc = super::Adc::new(0b0001_1100_0000_0001);
    adc.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0xfe);
    assert_eq!(status_register.get_carry(), 1);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
  }

  #[test]
  fn adc_0xff_0x01_returns0x00_with_status_registers() {
    let testbed = init(vec![(0, 0xff), (1, 0x01)]);

    let adc = super::Adc::new(0b0001_1100_0000_0001);
    adc.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x00);
    assert_eq!(status_register.get_carry(), 1);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn adc_0x01_0x02_carry_returns0x04_with_status_registers() {
    let testbed = init(vec![(0, 0x01), (1, 0x02)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let adc = super::Adc::new(0b0001_1100_0000_0001);
    adc.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x04);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }
}
