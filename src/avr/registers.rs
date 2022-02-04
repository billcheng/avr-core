#[derive(Debug)]
pub struct Registers {
  reg: Vec<u8>,
  stack_pointer: u32,
  z: u16,
}

impl Registers {
  pub fn new() -> Self {
    Self {
      reg: vec![0; 32],
      stack_pointer: 0,
      z: 0,
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
    self.z
  }

  pub fn set_z(&mut self, value: u16) {
    self.z = value;
  }
}
