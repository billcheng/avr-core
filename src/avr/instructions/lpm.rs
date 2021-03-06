use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;
use crate::avr::read_only_memory::ReadOnlyMemory;

pub struct Lpm {}

impl Lpm {
  pub fn new() -> Self {
    Self {}
  }
}

impl Instruction for Lpm {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
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

    registers.set(0, code_data);

    (3, None)
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn lpm_z_0x0002() {
    let testbed = init(vec![]);
    {
      let mut code_memory = testbed.code_memory.borrow_mut();
      code_memory.load(&vec![0x1011, 0x1213, 0x1415, 0x1617, 0x1819, 0x1a1b]);

      let mut registers = testbed.registers.borrow_mut();
      registers.set_z(0x0002);
    }

    let op = super::Lpm::new();
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(0), 0x13);
  }
}
