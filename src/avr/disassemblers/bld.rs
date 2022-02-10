use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::bld::Bld;

impl Disassembler for Bld {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("BLD"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.b)),
    )
  }
}
