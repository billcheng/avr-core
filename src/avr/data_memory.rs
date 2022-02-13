#![allow(dead_code)]

use crate::avr::random_access_memory::RandomAccessMemory;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DataMemory {
  memory: Vec<u8>,
}

pub type DataMemoryPtr = Rc<RefCell<DataMemory>>;

impl DataMemory {
  pub fn new(size: usize) -> Self {
    Self {
      memory: vec![0; size],
    }
  }

  pub fn load(&mut self, data: &Vec<u8>) {
    let size = data.len().min(self.memory.len());
    for idx in 0..size {
      self.memory[idx] = data[idx];
    }
  }

  pub fn get_all(&self) -> Vec<u8> {
    self.memory.clone()
  }
}

impl RandomAccessMemory for DataMemory {
  fn read(&self, address: u32) -> u8 {
    self.memory[address as usize]
  }

  fn write(&mut self, address: u32, value: u8) {
    self.memory[address as usize] = value;
  }
}

pub fn create_data_memory_ptr(size: usize) -> DataMemoryPtr {
  Rc::new(RefCell::new(DataMemory::new(size)))
}
