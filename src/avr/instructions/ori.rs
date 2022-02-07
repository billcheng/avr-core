use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Ori {
  k: u8,
  d: usize,
}

impl Ori {
  pub fn new(opcode: u16) -> Self {
    let d = 16 + ((opcode & 0b0000_0000_1111_0000) >> 4);
    let k = (((opcode & 0b0000_1111_0000_0000) >> 4) as u8) | ((opcode & 0x0f) as u8);

    Self {
      k,
      d: d as usize,
    }
  }
}

impl Instruction for Ori {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d);
    let result = rd | self.k;

    let r7 = (result >> 7 & 1) != 0;

    let overflow = false;
    let negative = r7;
    let sign = negative ^ overflow;
    let zero = result == 0;

    registers.set(self.d, result);

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_sign(sign);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn ori_r31_0x55_returns_0xff() {
    let testbed = init(vec![(31, 0xaa)]);

    let op = super::Ori::new(0b0110_0101_1111_0101);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0xff);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
  }

  #[test]
  fn ori_r31_0x55_returns_0x5f() {
    let testbed = init(vec![(31, 0x0a)]);

    let op = super::Ori::new(0b0110_0101_1111_0101);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x5f);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn ori_r31_0x00_returns_0x00() {
    let testbed = init(vec![(31, 0x00)]);

    let op = super::Ori::new(0b0110_0000_1111_0000);
    op.execute(super::InstructionData {
      status_register: testbed.status_register.clone(),
      registers: testbed.registers.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x00);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_sign(), 0);
  }
}
