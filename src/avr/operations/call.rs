use crate::avr::core_type::CoreType;
use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Call {
  k: u32,
  displacement: i32,
}

impl Call {
  pub fn new(core_type: &CoreType, opcode: u16, next_opcode: u16) -> Self {
    let k1 = ((opcode & 0b0000_0001_1111_0000) >> 3) | opcode & 0b0000_0000_0000_0001;
    let k = ((k1 as u32) << 16) | (next_opcode as u32);

    Self {
      k,
      displacement: match core_type {
        CoreType::Bits16 => -2,
        CoreType::Bits22 => -3,
      },
    }
  }
}

impl Operation for Call {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let stack_data = execution_data.pc + 2;

    let mut registers = execution_data.registers.borrow_mut();
    let mut stack = execution_data.data_memory.borrow_mut();
    stack.write(registers.get_stack_pointer(), (stack_data & 0xff) as u8);
    stack.write(
      registers.get_stack_pointer() - 1,
      ((stack_data >> 8) & 0xff) as u8,
    );
    stack.write(
      registers.get_stack_pointer() - 2,
      ((stack_data >> 16) & 0xff) as u8,
    );
    registers.add_stack_pointer(self.displacement);
    Some(self.k)
  }
}

#[cfg(test)]
mod test {
  use crate::avr::test::test_init::init;
use crate::avr::core_type::CoreType;
  use crate::avr::data_memory::create_data_memory_ptr;
  use crate::avr::operation::Operation;
  use crate::avr::random_access_memory::RandomAccessMemory;

  #[test]
  fn call_0x345678_returns_0x345678() {
    let (registers_ptr, status_register_ptr, data_memory) = init(vec![]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set_stack_pointer(9);
    }

    let op = super::Call::new(&CoreType::Bits16, 0b1001_010_1101_0_111_0, 0x5678);
    let result = op.execute(super::ExecutionData {
      registers: registers_ptr,
      status_register: status_register_ptr,
      pc: 0x0001,
      data_memory,
    });

    assert_eq!(result, Some(0x345678));
  }

  #[test]
  fn call_16_bits_returns_stack_pointer_7() {
    let (registers_ptr, status_register_ptr, data_memory) = init(vec![]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set_stack_pointer(9);
    }

    let op = super::Call::new(&CoreType::Bits16, 0b1001_010_1101_0_111_0, 0x5678);
    op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0001,
      data_memory,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get_stack_pointer(), 7);
  }

  #[test]
  fn call_22_bits_returns_stack_pointer_6() {
    let (registers_ptr, status_register_ptr, data_memory) = init(vec![]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set_stack_pointer(9);
    }

    let op = super::Call::new(&CoreType::Bits22, 0b1001_010_1101_0_111_0, 0x5678);
    op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0001,
      data_memory,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get_stack_pointer(), 6);
  }

  #[test]
  fn call_0x345678_returns_stack_data() {
    let (registers_ptr, status_register_ptr, data_memory) = init(vec![]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set_stack_pointer(9);
    }

    let op = super::Call::new(&CoreType::Bits16, 0b1001_010_1101_0_111_0, 0x5678);
    op.execute(
      super::ExecutionData {
        registers: registers_ptr.clone(),
        status_register: status_register_ptr,
        pc: 0x12345,
        data_memory: data_memory.clone(),
      });

    let stack = data_memory.borrow();
    assert_eq!(stack.read(9), 0x47);
    assert_eq!(stack.read(8), 0x23);
    assert_eq!(stack.read(7), 0x01);
  }
}
