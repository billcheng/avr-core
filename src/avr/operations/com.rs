use crate::avr::operation::ExecutionData;
use crate::avr::operation::Operation;

pub struct Com {
  d: usize,
}

impl Com {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;

    Self { d }
  }
}

impl Operation for Com {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d);
    let result = 0xff - rd;
    registers.set(self.d, result);

    let r7 = (result >> 7 & 1) != 0;

    let overflow = false;
    let negative = r7;
    let zero = result == 0;
    let carry = true;
    let sign = negative ^ overflow;

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_sign(sign);
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_carry(carry);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;
  use crate::avr::test::test_init::init;

  #[test]
  fn com_r1() {
    let (registers_ptr, status_register_ptr, data_memory, io) = init(vec![]);
    {
      let mut registers = registers_ptr.borrow_mut();
      registers.set(1, 0x05);
    }

    let op = super::Com::new(0b1001_0100_0001_0000);
    op.execute(super::ExecutionData {
      registers: registers_ptr.clone(),
      status_register: status_register_ptr,
      pc: 0x0000,
      data_memory,
      io: io,
    });

    let registers = registers_ptr.borrow();
    assert_eq!(registers.get(1), 0xfa);
  }
}
