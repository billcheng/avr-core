use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::swap::Swap;

impl Disassembler for Swap {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SWAP"),
      Some(format!("R{}", self.d)),
      None,
    )
  }
}
