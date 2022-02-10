use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::read_only_memory::ReadOnlyMemory;

pub struct LpmRd {
  pub(in crate::avr) d: usize,
}

impl LpmRd {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    Self { d }
  }
}

impl Instruction for LpmRd {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let z = registers.get_z();
    let addr = z >> 1;
    let high_byte = z & 1 != 0;

    let code_memory = execution_data.code_memory.borrow();

    let ps = code_memory.read(addr as u32);
    let code_data = match high_byte {
      true => (ps >> 8 & 0xff) as u8,
      false => (ps & 0xff) as u8,
    };

    registers.set(self.d, code_data);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn lpm_r31_z_0x0002() {
    let testbed = init(vec![]);
    {
      let mut code_memory = testbed.code_memory.borrow_mut();
      code_memory.load(&vec![0x1011, 0x1213, 0x1415, 0x1617, 0x1819, 0x1a1b]);

      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(0x0002);
    }

    let op = super::LpmRd::new(0b1001_0001_1111_0100);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(31), 0x13);
  }
}
