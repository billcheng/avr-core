#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

pub type IoPtr = Rc<RefCell<Io>>;

#[derive(Clone)]
pub struct Io {
  data: Vec<u8>,
}

impl Io {
  pub fn new() -> Self {
    Self { data: vec![0; 64] }
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
  pub fn get_all(&self) -> Vec<u8> {
    self.data.clone()
  }
}
