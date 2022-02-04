use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

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

impl Operation for Andi {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
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
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn andi_r0_0x55_returns_0x00() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(super::ExecutionData {
      status_register: status_register_ptr,
      registers: registers_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(0), 0x00);
  }

  #[test]
  fn andi_r0_0x55_returns_zero() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(super::ExecutionData {
      status_register: status_register_ptr.clone(),
      registers: registers_ptr,
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn andi_r0_0x55_returns_negative() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_1111_0000_1111);
    and.execute(super::ExecutionData {
      status_register: status_register_ptr.clone(),
      registers: registers_ptr,
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn andi_r0_0x55_returns_overflow() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_0101_0000_0101);
    and.execute(super::ExecutionData {
      status_register: status_register_ptr.clone(),
      registers: registers_ptr,
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn andi_r0_0xff_returns_sign() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(0, 0xaa)]);

    let and = super::Andi::new(0b0111_1111_0000_1111);
    and.execute(super::ExecutionData {
      status_register: status_register_ptr.clone(),
      registers: registers_ptr,
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_sign(), 1);
  }
}
