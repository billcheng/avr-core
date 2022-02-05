use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Ijmp {
}

impl Ijmp {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl Operation for Ijmp {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let registers = execution_data.registers.borrow();

    Some(registers.get_z() as u32)
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn ijmp_0x1234_changes_pc_to_0x1234() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set_z(0x1234);
    }

    let op = super::Ijmp::new();
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr,
      pc: 0x0000,
      data_memory: data_memory,
      io: io,
    });

    assert_eq!(result, Some(0x1234));
  }
}
