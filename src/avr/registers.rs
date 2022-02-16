#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

pub type RegistersPtr = Rc<RefCell<Registers>>;

#[derive(Debug, Clone)]
pub struct Registers {
  reg: Vec<u8>,
  stack_pointer: u32,
}

impl Registers {
  pub fn new(stack_pointer: u32) -> Self {
    Self {
      reg: vec![0; 32],
      stack_pointer: match stack_pointer > 0 {
        true => stack_pointer - 1,
        false => 0,
      },
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

  fn get_word(&self, index: usize) -> u16 {
    (self.get(index) as u16) | ((self.get(index + 1) as u16) << 8)
  }

  pub fn get_x(&self) -> u16 {
    self.get_word(26)
  }

  pub fn get_y(&self) -> u16 {
    self.get_word(28)
  }

  pub fn get_z(&self) -> u16 {
    self.get_word(30)
  }

  fn set_word(&mut self, index: usize, value: u16) {
    self.set(index, (value & 0xff) as u8);
    self.set(index + 1, (value >> 8) as u8);
  }

  pub fn set_x(&mut self, value: u16) {
    self.set_word(26, value);
  }

  pub fn set_y(&mut self, value: u16) {
    self.set_word(28, value);
  }

  pub fn set_z(&mut self, value: u16) {
    self.set_word(30, value);
  }

  pub fn get_all(&self) -> Vec<u8> {
    self.reg.clone()
  }

  pub fn reset(&mut self, size: usize) {
    self.reg.fill(0);
    self.stack_pointer = match size == 0 {
      true => 0,
      false => (size - 1) as u32,
    };
  }
}