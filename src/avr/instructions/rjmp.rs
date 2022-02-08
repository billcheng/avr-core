use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Rjmp {
  k: i16,
}

impl Rjmp {
  pub fn new(opcode: u16) -> Self {
    let k = (opcode & 0b0000_1111_1111_1111
      | match opcode & 0b0000_1000_0000_0000 {
        0b0000_0000_0000_0000 => 0,
        _ => 0b1111_0000_0000_0000,
      }) as i16;

    Self { k }
  }
}

impl Instruction for Rjmp {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let pc = execution_data.pc;
    let next_pc = pc as i64 + 1;
    let result = next_pc + self.k as i64;

    Some(result as u32)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn rjmp_1000() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(0xff);
    }

    let op = super::Rjmp::new(0b1101_0011_1110_1000);
    let result = op.execute(super::InstructionData {
      pc: 1500,
      ..testbed
    });

    assert_eq!(result, Some(2501));
  }

  #[test]
  fn rjmp_neg1000() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_stack_pointer(0xff);
    }

    let op = super::Rjmp::new(0b1101_1100_0001_1000);
    let result = op.execute(super::InstructionData {
      pc: 1500,
      ..testbed
    });

    assert_eq!(result, Some(501));
  }
}
