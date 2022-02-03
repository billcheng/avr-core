pub struct Registers {
  reg: Vec<u8>,
}

impl Registers {
  pub fn new() -> Self {
    Self {
      reg: vec![0; 32],
    }
  }

  pub fn get(&self, index: usize) -> u8 {
    self.reg[index]
  }

  pub fn set(&mut self, index: usize, value: u8) {
    self.reg[index] = value;
  }
}