use crate::avr::read_only_memory::ReadOnlyMemory;

pub struct CodeMemory {
  memory: Vec<u16>,
}

impl CodeMemory {
  pub fn new(size: usize) -> Self {
    Self {
      memory: vec![0; size],
    }
  }

  pub fn load(&mut self, data: &Vec<u16>) {
    let size = data.len().min(self.memory.len());
    for idx in 0..size {
      self.memory[idx] = data[idx];
    }
  }
}

impl ReadOnlyMemory for CodeMemory {
  fn read(&self, address: u32) -> u16 {
    self.memory[address as usize]
  }
}
