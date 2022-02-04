use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Cpi {
  k: u8,
  d: usize,
}

impl Cpi {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0000_1111_0000) >> 4) as usize;
    let k = ((opcode & 0b0000_0000_0000_1111) | ((opcode & 0b0000_1111_0000_0000) >> 4)) as u8;

    Self { d, k }
  }
}

impl Operation for Cpi {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();

    let rd = registers.get(self.d) as i16;
    let k = self.k as i16;
    let result = rd - k;

    let rd3 = (rd >> 3) & 1 != 0;
    let k3 = (k >> 3) & 1 != 0;
    let r3 = (result >> 3) & 1 != 0;
    let r7 = (result >> 7 & 1) != 0;
    let k7 = (k >> 7 & 1) != 0;
    let rd7 = (rd >> 7 & 1) != 0;

    let half_carry = !rd3 & k3 | k3 & r3 | r3 & !rd3;
    let overflow = rd7 & !k7 & !r7 | !rd7 & k7 & r7;
    let negative = r7;
    let sign = negative ^ overflow;
    let zero = result == 0;
    let carry = !rd7 & k7 | k7 & r7 | r7 & !rd7;

    let mut status_register = execution_data.status_register.borrow_mut();
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
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn cpi_r1_0xcc_return_z() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(1, 0xcc)]);

    let op = super::Cpi::new(0b0011_1100_0001_1100);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io: io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn cp_r1_0x04_return_hc() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(1, 0x00)]);

    let op = super::Cpi::new(0b0011_0000_0001_0100);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io: io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn cp_r1_0x01_return_s() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(1, 0xff)]);

    let op = super::Cpi::new(0b0011_0000_0001_0001);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io: io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn cp_r1_0x01_return_nc() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(1, 0x3)]);

    let op = super::Cpi::new(0b0011_0000_0001_0001);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io: io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_half_carry(), 0);
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn cp_r1_0x01_return_o() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(1, 0x80)]);

    let op = super::Cpi::new(0b0011_0000_0001_0001);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io: io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 1);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_half_carry(), 1);
    assert_eq!(status_register.get_carry(), 0);
  }
}
