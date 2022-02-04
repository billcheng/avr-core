use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::io::IoPtr;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;
use core::cell::RefCell;
use std::rc::Rc;

pub struct ExecutionData {
  pub status_register: Rc<RefCell<StatusRegister>>,
  pub registers: Rc<RefCell<Registers>>,
  pub pc: u32,
  pub data_memory: DataMemoryPtr,
  pub io: IoPtr,
}

pub trait Operation {
  fn execute(&self, execution_data: ExecutionData) -> Option<u32>;
}
