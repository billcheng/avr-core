use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction::InstructionResult;

pub struct Mul {
  pub(in crate::avr) d: usize,
  pub(in crate::avr) r: usize,
}

impl Mul {
  pub fn new(opcode: u16) -> Self {
    let d = ((opcode & 0b0000_0001_1111_0000) >> 4) as usize;
    let r = (opcode & 0b0000_0000_0000_1111 | ((opcode & 0b0000_0010_0000_0000) >> 5)) as usize;

    Self { d, r }
  }
}

impl Instruction for Mul {
  fn execute(&self, execution_data: InstructionData) -> InstructionResult {
    let mut registers = execution_data.registers.borrow_mut();

    let rd = registers.get(self.d) as u16;
    let rr = registers.get(self.r) as u16;
    let result = rd * rr;

    let carry = result & 0x8000 != 0;
    let zero = result == 0;

    registers.set(0, (result & 0xff) as u8);
    registers.set(1, ((result >> 8) & 0xff) as u8);

    let mut status_register = execution_data.status_register.borrow_mut();
    status_register.set_carry(carry);
    status_register.set_zero(zero);

    (2, None) // AVRrc=NA
  }
}

#[cfg(test)]
mod test {
  use super::Instruction;
  use crate::avr::test::test_init::init;

  #[test]
  fn mul_r3_r4_returns_0x03a8() {
    let testbed = init(vec![(3, 0x12), (4, 0x34)]);

    let op = super::Mul::new(0b1001_1100_0011_0100);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0xa8);
    assert_eq!(registers.get(1), 0x03);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 0);
  }

  #[test]
  fn mul_r3_r4_returns_0x0000() {
    let testbed = init(vec![(3, 0x12), (4, 0x0)]);

    let op = super::Mul::new(0b1001_1100_0011_0100);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x00);
    assert_eq!(registers.get(1), 0x00);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn mul_r3_r4_returns_0xef42() {
    let testbed = init(vec![(3, 0xf5), (4, 0xfa)]);

    let op = super::Mul::new(0b1001_1100_0011_0100);
    op.execute(super::InstructionData {
      registers: testbed.registers.clone(),
      status_register: testbed.status_register.clone(),
      ..testbed
    });

    let registers = testbed.registers.borrow();
    let status_register = testbed.status_register.borrow();
    assert_eq!(registers.get(0), 0x42);
    assert_eq!(registers.get(1), 0xef);
    assert_eq!(status_register.get_carry(), 1);
    assert_eq!(status_register.get_zero(), 0);
  }
}
