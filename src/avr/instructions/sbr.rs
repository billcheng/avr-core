use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;

pub struct Sbr {
  d: usize,
  k: u8,
}

impl Sbr {
  pub fn new(opcode: u16) -> Self {
    let d = 16 + ((opcode & 0b0000_0000_1111_0000) >> 4) as usize;
    let k = (opcode & 0b0000_0000_0000_1111 | ((opcode & 0b0000_1111_0000_0000) >> 4)) as u8;

    Self { d, k }
  }
}

impl Instruction for Sbr {
  fn execute(&self, execution_data: InstructionData) -> Option<u32> {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d);
    let result = rd | self.k;

    let r7 = result & 0x80 != 0;

    let overflow = false;
    let negative = r7;
    let sign = negative ^ overflow;
    let zero = result == 0;

    registers.set(self.d, result);

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_sign(sign);
    status_register.set_zero(zero);

    None
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn sbr_r16_0x55_returns_0xff() {
    let testbed = init(vec![(16, 0xaa)]);

    let op = super::Sbr::new(0b0110_0101_0000_0101);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(16), 0xff);
    assert_eq!(status_register.get_negative(), 1);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 1);
    assert_eq!(status_register.get_overflow(), 0);
  }

  #[test]
  fn sbr_r31_0x00_returns_0x00() {
    let testbed = init(vec![(16, 0x00)]);

    let op = super::Sbr::new(0b0110_0000_1111_0000);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(31), 0x00);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 1);
    assert_eq!(status_register.get_sign(), 0);
    assert_eq!(status_register.get_overflow(), 0);
  }
}
