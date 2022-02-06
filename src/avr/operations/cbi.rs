use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Cbi {
  a: usize,
  b: usize,
}

impl Cbi {
  pub fn new(opcode: u16) -> Self {
    let a = ((opcode & 0b0000_0000_1111_1000) >> 3) as usize;
    let b = (opcode & 0b0000_0000_0000_0111) as usize;

    Self { a, b }
  }
}

impl Operation for Cbi {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut io = execution_data.io.borrow_mut();

    io.clear_bit(self.a, self.b);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn cbi_clear_io5_bit7() {
    let testbed = init(vec![]);
    {
      let mut io = testbed.io.borrow_mut();
      io.set(5, 0xff);
    }

    let op = super::Cbi::new(0b1001_1000_0010_1111);
    op.execute(super::ExecutionData {
      io: testbed.io.clone(),
      ..testbed
    });

    let io = testbed.io.borrow();
    assert_eq!(io.get(5), 0b0111_1111);
  }
}
