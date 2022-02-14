use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Out {
  pub(in crate::avr) a: usize,
  pub(in crate::avr) r: usize,
}

impl Out {
  pub fn new(opcode: u16) -> Self {
    let r = (opcode & 0b0000_0001_1111_0000) >> 4;
    let a = (((opcode & 0b0000_0110_0000_0000) >> 5) as u8) | ((opcode & 0x0f) as u8);

    Self {
      a: a as usize,
      r: r as usize,
    }
  }
}

impl Instruction for Out {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let registers = execution_data.registers.borrow();

    let rr = registers.get(self.r);

    let mut io_registers = execution_data.io.borrow_mut();
    io_registers.set(self.a, rr);

    (1, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn out_63_r31() {
    let testbed = init(vec![(31, 0xaa)]);

    let op = super::Out::new(0b1011_1111_1111_1111);
    op.execute(super::InstructionData {
      io: testbed.io.clone(),
      ..testbed
    });

    let io = testbed.io.borrow();
    assert_eq!(io.get(63), 0xaa);
  }
}
