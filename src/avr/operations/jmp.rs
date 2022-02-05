use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Jmp {
  k: u32,
}

impl Jmp {
  pub fn new(opcode: u16, next_opcode: u16) -> Self {
    let k1 = ((opcode & 0b0000_0001_1111_0000) >> 3) | opcode & 0b0000_0000_0000_0001;
    let k = ((k1 as u32) << 16) | (next_opcode as u32);

    Self {
      k,
    }
  }
}

impl Operation for Jmp {
  fn execute(&self, _execution_data: ExecutionData) -> Option<u32> {
    Some(self.k)
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn jmp_0x345678_returns_0x345678() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);

    let op = super::Jmp::new(0b1001_010_1101_0_110_0, 0x5678);
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr,
      pc: 0x0001,
      data_memory,
      io,
    });

    assert_eq!(result, Some(0x345678));
  }
}