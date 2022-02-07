use crate::avr::code_memory::CodeMemoryPtr;
use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::io::IoPtr;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;
use core::cell::RefCell;
use std::rc::Rc;

pub struct InstructionData {
  pub status_register: Rc<RefCell<StatusRegister>>,
  pub registers: Rc<RefCell<Registers>>,
  pub pc: u32,
  pub data_memory: DataMemoryPtr,
  pub io: IoPtr,
  pub code_memory: CodeMemoryPtr,
}

pub trait Instruction {
  fn execute(&self, execution_data: InstructionData) -> Option<u32>;
}
