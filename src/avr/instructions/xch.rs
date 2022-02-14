use crate::avr::instruction::InstructionResult;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct Xch {
  pub(in crate::avr) d: usize,
}

impl Xch {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Instruction for Xch {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();
    let z = registers.get_z();
    let rd = registers.get(self.d);

    let mut data_memory = execution_data.data_memory.borrow_mut();
    let ds = data_memory.read(z as u32);

    registers.set(self.d, ds);
    data_memory.write(z as u32, rd);

    (2, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn xch_z0x0007_r5_0xaa() {
    let testbed = init(vec![(5,0xaa)]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(7);

      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(7, 0x55)
    }

    let op = super::Xch::new(0b1001_0010_0101_1100);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      data_memory: testbed.data_memory.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let data_memory = testbed.data_memory.borrow();
    assert_eq!(registers.get(5), 0x55);
    assert_eq!(data_memory.read(7), 0xaa);
  }
}
