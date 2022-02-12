#![allow(dead_code)]
#[derive(Clone)]
pub struct StatusRegister {
  carry: bool,
  zero: bool,
  negative: bool,
  overflow: bool,
  sign: bool,
  half_carry: bool,
  transfer: bool,
  interrupt: bool,
}

impl StatusRegister {
  pub fn new() -> Self {
    Self {
      carry: false,
      zero: false,
      negative: false,
      overflow: false,
      sign: false,
      half_carry: false,
      transfer: false,
      interrupt: false,
    }
  }

  pub fn get_carry(&self) -> u8 {
    get_flag(self.carry)
  }

  pub fn get_half_carry(&self) -> u8 {
    get_flag(self.half_carry)
  }

  pub fn get_overflow(&self) -> u8 {
    get_flag(self.overflow)
  }

  pub fn get_zero(&self) -> u8 {
    get_flag(self.zero)
  }

  pub fn get_negative(&self) -> u8 {
    get_flag(self.negative)
  }

  pub fn get_sign(&self) -> u8 {
    get_flag(self.sign)
  }

  pub fn get_transfer(&self) -> u8 {
    get_flag(self.transfer)
  }

  pub fn get_interrupt(&self) -> u8 {
    get_flag(self.interrupt)
  }

  pub fn get(&self, bit: usize) -> u8 {
    match bit {
      0 => self.get_carry(),
      1 => self.get_zero(),
      2 => self.get_negative(),
      3 => self.get_overflow(),
      4 => self.get_sign(),
      5 => self.get_half_carry(),
      6 => self.get_transfer(),
      7 => self.get_interrupt(),
      _ => panic!("Unknown status bit: {}", bit),
    }
  }

  pub fn set(&mut self, bit: usize, value: bool) {
    match bit {
      0 => self.set_carry(value),
      1 => self.set_zero(value),
      2 => self.set_negative(value),
      3 => self.set_overflow(value),
      4 => self.set_sign(value),
      5 => self.set_half_carry(value),
      6 => self.set_transfer(value),
      7 => self.set_interrupt(value),
      _ => panic!("Unknown status bit: {}", bit),
    };
  }

  pub fn set_half_carry(&mut self, value: bool) {
    self.half_carry = value;
  }

  pub fn set_overflow(&mut self, value: bool) {
    self.overflow = value;
  }

  pub fn set_negative(&mut self, value: bool) {
    self.negative = value;
  }

  pub fn set_zero(&mut self, value: bool) {
    self.zero = value;
  }

  pub fn set_carry(&mut self, value: bool) {
    self.carry = value;
  }

  pub fn set_sign(&mut self, value: bool) {
    self.sign = value;
  }

  pub fn set_transfer(&mut self, value: bool) {
    self.transfer = value;
  }

  pub fn set_interrupt(&mut self, value: bool) {
    self.interrupt = value;
  }

  pub fn get_byte(&self) -> u8 {
    self.get_carry() | self.get_zero() << 1 | self.get_negative() << 2 |
      self.get_overflow() << 3 | self.get_sign() << 4 | self.get_half_carry() << 5 |
      self.get_transfer() << 6 | self.get_interrupt() << 7
  }
}

fn get_flag(flag: bool) -> u8 {
  match flag {
    true => 1,
    _ => 0,
  }
}
