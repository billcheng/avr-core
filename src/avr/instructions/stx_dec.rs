use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct StxDec {
  pub(in crate::avr) r: usize,
}

impl StxDec {
  pub fn new(opcode: u16) -> Self {
    let r = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { r }
  }
}

impl Instruction for StxDec {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let rr = registers.get(self.r);
    let x = (registers.get_x() as i32 - 1) as u16;

    let mut data_memory = execution_data.data_memory.borrow_mut();
    data_memory.write(x as u32, rr);
    registers.set_x(x);

    (2, None) // AVRe=2, AVRxm=2, AVRxt=1, AVRrc=2
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use super::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn stxdec_0x00ff_r31() {
    let testbed = init(vec![(31, 0xaa)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_x(0x00ff);
    }

    let op = super::StxDec::new(0b1001_0011_1111_1110);
    op.execute(super::InstructionData {
      data_memory: testbed.data_memory.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let data_memory = testbed.data_memory.borrow();
    let registers = testbed.registers.borrow();
    assert_eq!(data_memory.read(0x00fe), 0xaa);
    assert_eq!(registers.get_x(), 0x00fe);
  }
}
