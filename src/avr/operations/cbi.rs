use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Cbi {
  a: usize,
  b: usize,
}

impl Cbi {
  pub fn new(opcode: u16) -> Self {
    let a = ((opcode & 0b0000_0000_1111_1000) >> 3) as usize;
    let b = (opcode & 0b0000_0000_0000_0111) as usize;

    Self { a, b }
  }
}

impl Operation for Cbi {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    None
  }
}

// #[cfg(test)]
// mod test {
//   use crate::avr::core_type::CoreType;
//   use crate::avr::data_memory::create_data_memory_ptr;
//   use crate::avr::operation::Operation;
//   use crate::avr::random_access_memory::RandomAccessMemory;

// }
