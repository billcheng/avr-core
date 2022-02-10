use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::com::Com;

impl Disassembler for Com {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("COM"), Some(format!("R{}", self.d)), None)
  }
}
