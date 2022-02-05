#[derive(Debug)]
pub struct Registers {
  reg: Vec<u8>,
  stack_pointer: u32,
}

impl Registers {
  pub fn new() -> Self {
    Self {
      reg: vec![0; 32],
      stack_pointer: 0,
    }
  }

  pub fn get(&self, index: usize) -> u8 {
    self.reg[index]
  }

  pub fn set(&mut self, index: usize, value: u8) {
    self.reg[index] = value;
  }

  pub fn set_stack_pointer(&mut self, value: u32) {
    self.stack_pointer = value;
  }

  pub fn get_stack_pointer(&self) -> u32 {
    self.stack_pointer
  }

  pub fn add_stack_pointer(&mut self, value: i32) {
    let result = self.stack_pointer as i32 + value;
    self.stack_pointer = result as u32;
  }

  pub fn get_z(&self) -> u16 {
    (self.get(30) as u16) | ((self.get(31) as u16) << 8)
  }

  pub fn set_z(&mut self, value: u16) {
    self.set(30, (value & 0xff) as u8);
    self.set(31, (value >> 8) as u8);
  }

  pub fn get_x(&self) -> u16 {
    (self.get(26) as u16) | ((self.get(27) as u16) << 8)
  }

  pub fn set_x(&mut self, value: u16) {
    self.set(26, (value & 0xff) as u8);
    self.set(27, (value >> 8) as u8);
  }
}
