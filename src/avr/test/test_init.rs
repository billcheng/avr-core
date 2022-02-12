use crate::avr::instruction::InstructionData;
use crate::avr::code_memory::CodeMemory;
use crate::avr::data_memory::create_data_memory_ptr;
use crate::avr::io::Io;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init(register_data: Vec<(usize, u8)>) -> InstructionData {
  let registers_ptr: Rc<RefCell<Registers>>;
  let status_register_ptr: Rc<RefCell<StatusRegister>>;

  let mut registers = Registers::new(1);
  for (index, value) in register_data {
    registers.set(index, value);
  }
  registers_ptr = Rc::new(RefCell::new(registers));

  let status_register = StatusRegister::new();
  status_register_ptr = Rc::new(RefCell::new(status_register));

  let data_memory = create_data_memory_ptr(256);
  let io = Rc::new(RefCell::new(Io::new()));
  let code_memory = Rc::new(RefCell::new(CodeMemory::new(256)));

  InstructionData {
    registers: registers_ptr,
    status_register: status_register_ptr,
    data_memory,
    io,
    pc: 0,
    code_memory,
  }
}
