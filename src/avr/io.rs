use std::cell::RefCell;
use std::rc::Rc;

pub type IoPtr = Rc<RefCell<Io>>;

pub struct Io {
  data: Vec<u8>,
}

impl Io {
  pub fn new() -> Self {
    Self { data: vec![0; 32] }
  }

  pub fn get(&self, index: usize) -> u8 {
    self.data[index]
  }

  pub fn set(&mut self, index: usize, value: u8) {
    self.data[index] = value;
  }

  pub fn set_bit(&mut self, index: usize, bit: usize) {
    let mask = 1 << bit;
    self.data[index] |= mask;
  }

  pub fn clear_bit(&mut self, index: usize, bit: usize) {
    let mask = 1 << bit;
    self.data[index] &= !mask;
  }
}
