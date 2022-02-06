use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Ldi {
  d: usize,
  k: u8,
}

impl Ldi {
  pub fn new(opcode: u16) -> Self {
    let d = (opcode & 0b0000_0000_1111_0000) >> 4;
    let k = (((opcode & 0b0000_1111_0000_0000) >> 4 | opcode & 0b0000_0000_0000_1111) & 0xff) as u8;

    Self { d: d as usize, k }
  }
}

impl Operation for Ldi {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();
    registers.set(self.d, self.k);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn ldi_r10_0x55_returns_0x55() {
    let testbed = init(vec![]);

    let op = super::Ldi::new(0b1110_0101_1010_0101);
    op.execute(super::ExecutionData {
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    assert_eq!(registers.get(10), 0x55);
  }
}
