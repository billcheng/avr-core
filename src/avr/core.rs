use crate::avr::code_memory::CodeMemory;
use crate::avr::core_type::CoreType;
use crate::avr::data_memory::DataMemory;
use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::instruction_manager::InstructionManager;
use crate::avr::read_only_memory::ReadOnlyMemory;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Core {
  core_type: CoreType,
  code_memory: Rc<RefCell<CodeMemory>>,
  status_register: StatusRegister,
  program_counter: u32,
  instruction: InstructionManager,
  registers: Registers,
  data_memory: DataMemoryPtr,
}

impl Core {
  pub fn new(core_type: CoreType, code_size: usize, data_size: usize) -> Self {
    let code_memory = Rc::new(RefCell::new(CodeMemory::new(code_size)));
    let data_memory = Rc::new(RefCell::new(DataMemory::new(data_size)));

    Self {
      core_type,
      program_counter: 0,
      code_memory,
      status_register: StatusRegister::new(),
      instruction: InstructionManager::new(),
      registers: Registers::new(),
      data_memory,
    }
  }

  pub fn load_code(&mut self, codes: &Vec<u16>) {
    let mut code_memory = self.code_memory.borrow_mut();
    code_memory.load(codes);
  }

  pub fn single_step(&mut self) {
    let code_memory = self.code_memory.borrow();
    let opcode = code_memory.read(self.program_counter);
    let next_opcode = code_memory.read(self.program_counter + 1);
    let operation = self.instruction.get(&self.core_type, opcode, next_opcode);
    let result = operation.execute(
      &mut self.status_register,
      &mut self.registers,
      self.program_counter,
      &self.data_memory,
    );
    self.program_counter = match result {
      None => self.program_counter + 1,
      Some(next_pc) => next_pc,
    };
  }
}
