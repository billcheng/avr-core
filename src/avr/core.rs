use crate::avr::registers::Registers;
use crate::avr::code_memory::CodeMemory;
use crate::avr::instruction_manager::InstructionManager;
use crate::avr::read_only_memory::ReadOnlyMemory;
use crate::avr::status_register::StatusRegister;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Core {
  code_memory: Rc<RefCell<CodeMemory>>,
  status_register: StatusRegister,
  program_counter: u16,
  instruction: InstructionManager,
  registers: Registers,
}

impl Core {
  pub fn new(code_size: usize, data_size: usize) -> Self {
    let code_memory = Rc::new(RefCell::new(CodeMemory::new(code_size)));

    Self {
      program_counter: 0,
      code_memory: code_memory,
      status_register: StatusRegister::new(),
      instruction: InstructionManager::new(),
      registers: Registers::new(),
    }
  }

  pub fn load_code(&mut self, codes: &Vec<u16>) {
    let mut code_memory = self.code_memory.borrow_mut();
    code_memory.load(codes);
  }

  pub fn single_step(&mut self) {
    let code_memory = self.code_memory.borrow();
    let opcode = code_memory.read(self.program_counter);
    self.program_counter += 1;

    let operation = self.instruction.get(opcode);
    operation.execute(&mut self.status_register, &mut self.registers, &self.program_counter);
  }
}
