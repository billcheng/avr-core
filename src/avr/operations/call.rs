use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Call {
  k: u32,
}

impl Call {
  pub fn new(opcode: u16, next_opcode: u16) -> Self {
    let k1 = ((opcode & 0b0000_0001_1111_0000) >> 3) | opcode & 0b0000_0000_0000_0001;
    let k = ((k1 as u32) << 16) | (next_opcode as u32);

    Self { k }
  }
}

impl Operation for Call {
  fn execute(
    &self,
    _status_register: &mut StatusRegister,
    _registers: &mut Registers,
    _pc: u32,
  ) -> Option<u32> {
    Some(self.k)
  }
}

#[cfg(test)]
mod test {
  use crate::avr::operation::Operation;

  #[test]
  fn call_0x345678_returns_0x345678() {
    let mut registers = super::Registers::new();
    let mut status_register = super::StatusRegister::new();

    let op = super::Call::new(0b1001_010_1101_0_111_0, 0x5678);
    let result = op.execute(&mut status_register, &mut registers, 0x0001);

    assert_eq!(result, Some(0x345678));
  }
}
