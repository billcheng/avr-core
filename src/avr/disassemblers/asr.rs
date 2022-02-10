use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::asr::Asr;

impl Disassembler for Asr {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("ASR"), Some(format!("R{}", self.d)), None)
  }
}
