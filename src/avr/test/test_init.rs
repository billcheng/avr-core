use crate::avr::data_memory::create_data_memory_ptr;
use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::io::Io;
use crate::avr::io::IoPtr;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init(
  register_data: Vec<(usize, u8)>,
) -> (
  Rc<RefCell<Registers>>,
  Rc<RefCell<StatusRegister>>,
  DataMemoryPtr,
  IoPtr,
) {
  let registers_ptr: Rc<RefCell<Registers>>;
  let status_register_ptr: Rc<RefCell<StatusRegister>>;

  let mut registers = Registers::new();
  for (index, value) in register_data {
    registers.set(index, value);
  }
  registers_ptr = Rc::new(RefCell::new(registers));

  let status_register = StatusRegister::new();
  status_register_ptr = Rc::new(RefCell::new(status_register));

  let data_memory = create_data_memory_ptr(256);
  let io = Rc::new(RefCell::new(Io::new()));

  (registers_ptr, status_register_ptr, data_memory, io)
}
