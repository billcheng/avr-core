use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Brbc {
  k: i8,
  s: usize,
}

impl Brbc {
  pub fn new(opcode: u16) -> Self {
    let k = (opcode & 0b0000_0011_1111_1000) >> 2;
    let s = opcode & 0b0000_0000_0000_0111;

    Self {
      k: k as i8 >> 1,
      s: s as usize,
    }
  }
}

impl Operation for Brbc {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let status_register = execution_data.status_register.borrow();

    let flag = status_register.get(self.s);
    let k = self.k as i8;

    match flag {
      0 => Some(((execution_data.pc as i64) + (k as i64) + 1) as u32),
      _ => None,
    }
  }
}

#[cfg(test)]
mod test {
  use crate::avr::data_memory::create_data_memory_ptr;
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn brbc_nc_0x0001_returns0x0001() {
    let (registers_ptr, status_register_ptr, data_memory) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Brbc::new(0b1111_0111_1111_1000);
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr,
      pc: 0x0001,
      data_memory,
    });

    assert_eq!(result, Some(1));
  }

  #[test]
  fn brbc_nc_0x0001_returns0x0003() {
    let (registers_ptr, status_register_ptr, data_memory) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_carry(false);
    }

    let op = super::Brbc::new(0b1111_0100_0000_1000);
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr,
      pc: 0x0001,
      data_memory,
    });

    assert_eq!(result, Some(3));
  }

  #[test]
  fn brbc_c_0x0001_returns_none() {
    let (registers_ptr, status_register_ptr, data_memory) = init(vec![]);
    {
      let mut status_register = status_register_ptr.borrow_mut();
      status_register.set_carry(true);
    }

    let op = super::Brbc::new(0b1111_0100_0000_1000);
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr,
      pc: 0x0001,
      data_memory,
    });

    assert_eq!(result, None);
  }
}
