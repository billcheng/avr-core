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
}

fn get_flag(flag: bool) -> u8 {
  match flag {
    true => 1,
    _ => 0,
  }
}
