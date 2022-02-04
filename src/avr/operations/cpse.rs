use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::util::opcode_size::Opcode;
use crate::avr::util::opcode_size::OpcodeSize;
use std::rc::Rc;

pub struct Cpse {
  r: usize,
  d: usize,
  next_opcode: u16,
  opcode_util: Rc<Opcode>,
}

impl Cpse {
  pub fn new(opcode: u16, next_opcode: u16, opcode_util: &Rc<Opcode>) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let r = ((opcode & 0b0000_0000_0000_1111) | ((opcode & 0b0000_0010_0000_0000) >> 5)) as usize;

    Self {
      d,
      r,
      next_opcode,
      opcode_util: opcode_util.clone(),
    }
  }
}

impl Operation for Cpse {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();

    let rd = registers.get(self.d);
    let rr = registers.get(self.r);

    if rd != rr {
      return None;
    }

    Some(
      execution_data.pc
        + match self.opcode_util.get_size(self.next_opcode) {
          2 => 3,
          1 => 2,
          _ => panic!("Invalid opcode size"),
        },
    )
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::operations::cpse::Rc;
  use crate::avr::test::test_init::init;
  use crate::avr::util::opcode_size::Opcode;

  #[test]
  fn cpse_not_equal_return_none() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(1, 0xcc), (2, 0xaa)]);

    let op = super::Cpse::new(0b0001_0000_0001_0010, 0x0000, &Rc::new(Opcode::new()));
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io: io,
    });

    assert_eq!(result, None);
  }

  #[test]
  fn cpse_equal_return_pc_2() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![(1, 0xcc), (2, 0xcc)]);

    let op = super::Cpse::new(0b0001_0000_0001_0010, 0x0000, &Rc::new(Opcode::new()));
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr.clone(),
      pc: 0x0000,
      data_memory,
      io: io,
    });

    assert_eq!(result, Some(2));
  }
}
