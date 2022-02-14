use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Sbci {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) k: usize,
}

impl Sbci {
  pub fn new(opcode: u16) -> Self {
    let d = 16 + ((opcode & 0b0000_0000_1111_0000) >> 4) as usize;
    let k = (opcode & 0b0000_0000_0000_1111 | ((opcode & 0b0000_1111_0000_0000) >> 4)) as usize;

    Self { d, k }
  }
}

impl Instruction for Sbci {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let mut status_register = execution_data.status_register.borrow_mut();

    let rd = registers.get(self.d);
    let c = status_register.get_carry();

    let result = (rd as i16 - self.k as i16 - c as i16) as u8;

    let rd3 = (rd & 0b000_1000) != 0;
    let rr3 = (self.k & 0b000_1000) != 0;
    let r3 = (result & 0b000_1000) != 0;
    let rd7 = (rd & 0b1000_0000) != 0;
    let rr7 = (self.k & 0b1000_0000) != 0;
    let r7 = (result & 0b1000_0000) != 0;

    let half_carry = !rd3 & rr3 | rr3 & r3 | r3 & !rd3;
    let overflow = rd7 & !rr7 & !r7 | !rd7 & rr7 & r7;
    let negative = r7;
    let sign = negative ^ overflow;
    let zero = result == 0 && status_register.get_zero() == 1;
    let carry = !rd7 & rr7 | rr7 & r7 | r7 & !rd7;

    registers.set(self.d, result);
    status_register.set_half_carry(half_carry);
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_sign(sign);
    status_register.set_zero(zero);
    status_register.set_carry(carry);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn sbci_r30_0x0f_0x1_nc() {
    let testbed = init(vec![(30, 0x0f)]);

    let op = super::Sbci::new(0b0100_0000_1110_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0x0e);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
  }

  #[test]
  fn sbc_r30_0x0f_0x1_c() {
    let testbed = init(vec![(30, 0x0f)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Sbci::new(0b0100_0000_1110_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0x0d);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
  }

  #[test]
  fn sbc_r30_0x10_0xf_nc() {
    let testbed = init(vec![(30, 0x10)]);

    let op = super::Sbci::new(0b0100_0000_1110_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 1);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_half_carry(), 1);
  }

  #[test]
  fn sbc_r30_0x10_0xf_c_nz() {
    let testbed = init(vec![(30, 0x10)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Sbci::new(0b0100_0000_1110_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_half_carry(), 1);
  }

  #[test]
  fn sbc_r30_0x10_0xf_c_z() {
    let testbed = init(vec![(30, 0x10)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
      status_register.set_zero(true);
    }

    let op = super::Sbci::new(0b0100_0000_1110_1111);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_half_carry(), 1);
  }

  #[test]
  fn sbc_r30_0x10_0x10_nc_z() {
    let testbed = init(vec![(30, 0x10)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_zero(true);
    }

    let op = super::Sbci::new(0b0100_0001_1110_0000);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
  }

  #[test]
  fn sbc_r30_0x10_0x10_nc_nz() {
    let testbed = init(vec![(30, 0x10)]);

    let op = super::Sbci::new(0b0100_0001_1110_0000);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
  }

  #[test]
  fn sbc_r30_0x10_0x11_nc_nz() {
    let testbed = init(vec![(30, 0x10)]);

    let op = super::Sbci::new(0b0100_0001_1110_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0xff);
    assert_eq!(status_register.get_carry(), 1);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_half_carry(), 1);
  }

  #[test]
  fn sbc_r30_0x80_0x01_nc() {
    let testbed = init(vec![(30, 0x80)]);

    let op = super::Sbci::new(0b1000_0000_1110_0001);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(30), 0x7f);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_overflow(), 1);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_half_carry(), 1);
  }
}
