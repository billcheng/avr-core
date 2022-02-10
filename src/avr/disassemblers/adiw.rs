use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::adiw::Adiw;

impl Disassembler for Adiw {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ADD"),
      Some(format!("R{}:R{}", self.d + 1, self.d)),
      Some(format!("{}", self.k)),
    )
  }
}
