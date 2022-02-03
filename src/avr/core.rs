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
  code_address: u16,
  code_read: bool,
  code_memory_observer: Vec<Rc<RefCell<dyn ReadOnlyMemory>>>,
  instruction: InstructionManager,
  registers: Registers,
}

impl Core {
  pub fn new(code_size: usize, data_size: usize) -> Self {
    let code_memory = Rc::new(RefCell::new(CodeMemory::new(code_size)));

    Self {
      code_address: 0,
      code_read: false,
      code_memory: code_memory.clone(),
      status_register: StatusRegister::new(),
      code_memory_observer: vec![code_memory],
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
    let opcode = code_memory.read(self.code_address);
    self.code_address += 1;

    let operation = self.instruction.get(opcode);
    operation.execute(&mut self.status_register, &mut self.registers);
  }
}
