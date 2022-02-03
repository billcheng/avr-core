use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::operation::Operation;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub struct Adiw {
  d: usize,
  k: u8,
}

impl Adiw {
  pub fn new(opcode: u16) -> Self {
    let d = 24 + (opcode & 0b0000_0000_0011_0000 >> 3);
    let decoded_k1 = opcode & 0b0000_0000_0000_1111;
    let decoded_k2 = opcode & 0b0000_0000_1100_0000 >> 2;
    let k = decoded_k1 | decoded_k2;

    Self {
      d: d as usize,
      k: k as u8,
    }
  }
}

impl Operation for Adiw {
  fn execute(
    &self,
    status_register: &mut StatusRegister,
    registers: &mut Registers,
    _pc: u32,
    _data_memory: &DataMemoryPtr,
  ) -> Option<u32> {
    let rd = registers.get(self.d) as u16 | ((registers.get(self.d + 1) as u16) << 8);
    let result = rd as u32 + self.k as u32;
    registers.set(self.d, (result & 0x00ff) as u8);
    registers.set(self.d + 1, ((result & 0xff00) >> 8) as u8);

    let rdh7 = (rd >> 15 & 1) != 0;
    let r15 = (result >> 15 & 1) != 0;

    let overflow = !rdh7 & r15;
    let negative = r15;
    let zero = result & 0xffff == 0;
    let carry = rdh7 & !r15;
    let sign = negative ^ overflow;

    status_register.set_overflow(overflow);
    status_register.set_negative(negative);
    status_register.set_zero(zero);
    status_register.set_carry(carry);
    status_register.set_sign(sign);

    None
  }
}

#[cfg(test)]
mod test {
  use crate::avr::data_memory::create_data_memory_ptr;
  use crate::avr::operation::Operation;

  #[test]
  fn adiw_r24_0x01_returns0x0200_with_status_registers() {
    let mut registers = super::Registers::new();
    registers.set(24, 0xff);
    registers.set(25, 0x01);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(registers.get(24), 0x00);
    assert_eq!(registers.get(25), 0x02);
    assert_eq!(status_register.get_carry(), 0);
    assert_eq!(status_register.get_overflow(), 0);
    assert_eq!(status_register.get_negative(), 0);
    assert_eq!(status_register.get_zero(), 0);
    assert_eq!(status_register.get_sign(), 0);
  }

  #[test]
  fn adiw_r24_0x01_returns_carry() {
    let mut registers = super::Registers::new();
    registers.set(24, 0xff);
    registers.set(25, 0xff);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_carry(), 1);
  }

  #[test]
  fn adiw_r24_0x01_returns_zero() {
    let mut registers = super::Registers::new();
    registers.set(24, 0xff);
    registers.set(25, 0xff);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_zero(), 1);
  }

  #[test]
  fn adiw_r24_0x01_returns_negative() {
    let mut registers = super::Registers::new();
    registers.set(24, 0xff);
    registers.set(25, 0xef);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_negative(), 1);
  }

  #[test]
  fn adiw_r24_0x01_returns_overflow() {
    let mut registers = super::Registers::new();
    registers.set(24, 0xff);
    registers.set(25, 0x7f);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_overflow(), 1);
  }

  #[test]
  fn adiw_r24_0x01_returns_sign() {
    let mut registers = super::Registers::new();
    registers.set(24, 0xff);
    registers.set(25, 0xef);
    let mut status_register = super::StatusRegister::new();
    let data_memory = create_data_memory_ptr(10);

    let adiw = super::Adiw::new(0b1001_0110_0000_0001);
    adiw.execute(&mut status_register, &mut registers, 0x0000, &data_memory);

    assert_eq!(status_register.get_sign(), 1);
  }
}
