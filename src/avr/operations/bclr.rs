use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Bclr {
  s: usize,
}

impl Bclr {
  pub fn new(opcode: u16) -> Self {
    let s = (opcode & 0b0000_0000_0111_0000) >> 4;

    Self { s: s as usize }
  }
}

impl Operation for Bclr {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set(self.s, false);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn bclr_0_clears_carry() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Bclr::new(0b1001_0100_1000_1000);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_carry(), 0);
  }

  #[test]
  fn bclr_1_clears_zero() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_zero(true);
    }

    let op = super::Bclr::new(0b1001_0100_1001_1000);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn bclr_2_clears_negative() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_negative(true);
    }

    let op = super::Bclr::new(0b1001_0100_1010_1000);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_negative(), 0);
  }

  #[test]
  fn bclr_3_clears_overflow() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_overflow(true);
    }

    let op = super::Bclr::new(0b1001_0100_1011_1000);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn bclr_4_clears_sign() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_sign(true);
    }

    let op = super::Bclr::new(0b1001_0100_1100_1000);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn bclr_5_clears_half_carry() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_half_carry(true);
    }

    let op = super::Bclr::new(0b1001_0100_1101_1000);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_half_carry(), 0);
  }

  #[test]
  fn bclr_6_clears_transfer() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_transfer(true);
    }

    let op = super::Bclr::new(0b1001_0100_1110_1000);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_transfer(), 0);
  }

  #[test]
  fn bclr_7_clears_interrupt() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_interrupt(true);
    }

    let op = super::Bclr::new(0b1001_0100_1111_1000);
    op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io,
    });

    let status_register = status_register_ptr.borrow();
    assert_eq!(status_register.get_interrupt(), 0);
  }
}
