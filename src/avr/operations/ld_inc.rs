use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;
use crate::avr::random_access_memory::RandomAccessMemory;

pub struct LdInc {
  d: usize,
}

impl LdInc {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Operation for LdInc {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    let x = registers.get_x();

    let data_memory = execution_data.data_memory.borrow();
    let ds = data_memory.read(x as u32);

    registers.set(self.d, ds);
    registers.set_x(((x as u32) + 1) as u16);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::random_access_memory::RandomAccessMemory;
  use crate::avr::test::test_init::init;

  #[test]
  fn ldinc_r5_0x0007_returns_0xfe() {
    let testbed = init(vec![]);
    {
      let mut registers = testbed.registers.borrow_mut();
      registers.set_x(7);

      let mut mem = testbed.data_memory.borrow_mut();
      mem.write(7, 0xfe)
    }

    let op = super::LdInc::new(0b1001_0000_0101_1101);
    op.execute(super::ExecutionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(5), 0xfe);
    assert_eq!(registers.get_x(), 8);
  }
}
