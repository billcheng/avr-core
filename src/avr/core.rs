#![allow(dead_code)]
use crate::avr::avr_type::AvrType;
use crate::avr::code_memory::CodeMemory;
use crate::avr::code_memory::CodeMemoryPtr;
use crate::avr::core_type::CoreType;
use crate::avr::data_memory::create_data_memory_ptr;
use crate::avr::data_memory::DataMemoryPtr;
use crate::avr::disassembler::Disassembler;
use crate::avr::instruction::Instruction;
use crate::avr::instruction::InstructionData;
use crate::avr::instruction_decoder::InstructionDecoder;
use crate::avr::io::Io;
use crate::avr::io::IoPtr;
use crate::avr::read_only_memory::ReadOnlyMemory;
use crate::avr::registers::Registers;
use crate::avr::registers::RegistersPtr;
use crate::avr::status_register::StatusRegister;
use crate::avr::status_register::StatusRegisterPtr;
use crate::avr::util::opcode_size::Opcode;
use crate::avr::util::opcode_size::OpcodeSize;
use std::cell::RefCell;
use std::rc::Rc;

trait Inst: Instruction + Disassembler {}

pub struct Core {
  core_type: CoreType,
  avr_type: AvrType,
  code_memory: CodeMemoryPtr,
  status_register: StatusRegisterPtr,
  program_counter: u32,
  instruction_decoder: InstructionDecoder,
  registers: RegistersPtr,
  data_memory: DataMemoryPtr,
  io: IoPtr,
  opcode_util: Rc<Opcode>,
  code_size: u32,
  cycles: u32,
}

impl Core {
  pub fn new(core_type: CoreType, avr_type: AvrType, code_size: usize, data_size: usize) -> Self {
    let code_memory = Rc::new(RefCell::new(CodeMemory::new(code_size)));
    let data_memory = create_data_memory_ptr(data_size);
    let io = Rc::new(RefCell::new(Io::new()));
    let opcode_util = Rc::new(Opcode::new());

    Self {
      core_type: core_type.clone(),
      avr_type: avr_type.clone(),
      program_counter: 0,
      code_memory,
      status_register: Rc::new(RefCell::new(StatusRegister::new())),
      instruction_decoder: InstructionDecoder::new(&opcode_util, &core_type, &avr_type),
      registers: Rc::new(RefCell::new(Registers::new(data_size as u32))),
      data_memory,
      io,
      opcode_util,
      code_size: code_size as u32,
      cycles: 0,
    }
  }

  pub fn load_code(&mut self, codes: &Vec<u16>) {
    let mut code_memory = self.code_memory.borrow_mut();
    code_memory.load(codes);
  }

  pub fn single_step(&mut self) {
    let code_memory = self.code_memory.borrow();
    let opcode = code_memory.read(self.program_counter);
    let next_pc = (self.program_counter + 1) % self.code_size;
    let next_opcode = code_memory.read(next_pc);
    let instruction = self.instruction_decoder.get(opcode, next_opcode);
    let (number_of_cycles, result) = instruction.execute(InstructionData {
      status_register: self.status_register.clone(),
      registers: self.registers.clone(),
      pc: self.program_counter,
      data_memory: self.data_memory.clone(),
      io: self.io.clone(),
      code_memory: self.code_memory.clone(),
    });
    self.cycles += number_of_cycles;
    self.program_counter = match result {
      None => next_pc,
      Some(absolute_pc) => absolute_pc,
    };
  }

  pub fn disassemble(&self, address: u32) -> (String, Option<String>, Option<String>) {
    let code_memory = self.code_memory.borrow();
    let opcode = code_memory.read(address);
    let next_address = (address + 1) % self.code_size;
    let next_opcode = code_memory.read(next_address);
    let instruction = self.instruction_decoder.get(opcode, next_opcode);
    instruction.disassemble(address)
  }

  pub fn get_instruction_size(&self, address: u32) -> usize {
    let code_memory = self.code_memory.borrow();
    let opcode = code_memory.read(address);
    self.opcode_util.get_size(opcode)
  }

  pub fn get_program_counter(&self) -> u32 {
    self.program_counter
  }

  pub fn get_registers(&self) -> RegistersPtr {
    self.registers.clone()
  }

  pub fn get_status_register(&self) -> StatusRegisterPtr {
    self.status_register.clone()
  }

  pub fn get_io(&self) -> IoPtr {
    self.io.clone()
  }

  pub fn get_data_memory(&self) -> DataMemoryPtr {
    self.data_memory.clone()
  }

  pub fn get_code_memory(&self) -> CodeMemoryPtr {
    self.code_memory.clone()
  }

  pub fn get_cycles(&self) -> u32 {
    self.cycles
  }
}
