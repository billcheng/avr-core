use crate::avr::operation::Instruction;
use crate::avr::operation::InstructionData;

pub struct Cpc {
  r: usize,
  d: usize,
}

impl Cpc {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let r = ((opcode & 0b0000_0000_0000_1111) | ((opcode & 0b0000_0010_0000_0000) >> 5)) as usize;

    Self { d, r }
  }
}

impl Instruction for Cpc {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();
    let mut status_register = execution_data.status_register.borrow_mut();

    let rd = registers.get(self.d) as i16;
    let rr = registers.get(self.r) as i16;
    let c = status_register.get_carry() as i16;
    let result = rd - rr - c;

    let rd3 = (rd >> 3) & 1 != 0;
    let rr3 = (rr >> 3) & 1 != 0;
    let r3 = (result >> 3) & 1 != 0;
    let r7 = (result >> 7 & 1) != 0;
    let rr7 = (rr >> 7 & 1) != 0;
    let rd7 = (rd >> 7 & 1) != 0;

    let half_carry = !rd3 & rr3 | rr3 & r3 | r3 & !rd3;
    let overflow = rd7 & !rr7 & !r7 | !rd7 & rr7 & r7;
    let negative = r7;
    let sign = negative ^ overflow;
    let zero = result == 0;
    let carry = !rd7 & rr7 | rr7 & r7 | r7 & !rd7;

    status_register.set_half_carry(half_carry);
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
  fn cp_r1_r2_nc_return_z() {
    let testbed = init(vec![(1, 0xcc), (2, 0xcc)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Cpc::new(0b0001_0100_0001_0010);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn cp_r1_r2_c_return_nz() {
    let testbed = init(vec![(1, 0xcc), (2, 0xcc)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Cpc::new(0b0001_0100_0001_0010);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn cp_r1_r2_nc_return_hc() {
    let testbed = init(vec![(1, 0x00), (2, 0x04)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Cpc::new(0b0001_0100_0001_0010);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn cp_r1_r2_c_return_z() {
    let testbed = init(vec![(1, 0x05), (2, 0x04)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Cpc::new(0b0001_0100_0001_0010);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn cp_r1_r2_nc_return_s() {
    let testbed = init(vec![(1, 0xff), (2, 0x01)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Cpc::new(0b0001_0100_0001_0010);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn cp_r1_r2_nc_return_nc() {
    let testbed = init(vec![(1, 0x3), (2, 0x01)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Cpc::new(0b0001_0100_0001_0010);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn cp_r1_r2_nc_return_o() {
    let testbed = init(vec![(1, 0x80), (2, 0x01)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Cpc::new(0b0001_0100_0001_0010);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 1);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn cp_r1_r2_c_return_0() {
    let testbed = init(vec![(1, 0x80), (2, 0x00)]);
    {
      let mut status_register = testbed.status_register.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Cpc::new(0b0001_0100_0001_0010);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let status_register = testbed.status_register.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 1);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_carry(), 0);
  }
}
