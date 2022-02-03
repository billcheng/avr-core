use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;

pub trait Operation {
  fn execute(
    &self,
    status_register: &mut StatusRegister,
    registers: &mut Registers,
    pc: u32,
    data_memory: &DataMemoryPtr,
  ) -> Option<u32>;
}
