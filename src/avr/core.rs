use crate::avr::code_memory::CodeMemory;
use crate::avr::core_type::CoreType;
use crate::avr::data_memory::create_data_memory_ptr;
use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::instruction_manager::InstructionManager;
use crate::avr::io::Io;
use crate::avr::io::IoPtr;
use crate::avr::operation::ExecutionData;
use crate::avr::read_only_memory::ReadOnlyMemory;
use crate::avr::registers::Registers;
use crate::avr::status_register::StatusRegister;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Core {
  core_type: CoreType,
  code_memory: Rc<RefCell<CodeMemory>>,
  status_register: Rc<RefCell<StatusRegister>>,
  program_counter: u32,
  instruction: InstructionManager,
  registers: Rc<RefCell<Registers>>,
  data_memory: DataMemoryPtr,
  io: IoPtr,
}

impl Core {
  pub fn new(core_type: CoreType, code_size: usize, data_size: usize) -> Self {
    let code_memory = Rc::new(RefCell::new(CodeMemory::new(code_size)));
    let data_memory = create_data_memory_ptr(data_size);
    let io = Rc::new(RefCell::new(Io::new()));

    Self {
      core_type,
      program_counter: 0,
      code_memory,
      status_register: Rc::new(RefCell::new(StatusRegister::new())),
      instruction: InstructionManager::new(),
      registers: Rc::new(RefCell::new(Registers::new())),
      data_memory,
      io,
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
    let result = operation.execute(ExecutionData {
      status_register: self.status_register.clone(),
      registers: self.registers.clone(),
      pc: self.program_counter,
      data_memory: self.data_memory.clone(),
      io: self.io.clone(),
    });
    self.program_counter = match result {
      None => self.program_counter + 1,
      Some(next_pc) => next_pc,
    };
  }
}
