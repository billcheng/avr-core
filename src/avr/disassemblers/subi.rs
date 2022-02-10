use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::subi::Subi;

impl Disassembler for Subi {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ST"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.k)),
    )
  }
}
