use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::andi::Andi;

impl Disassembler for Andi {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ANDI"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.k)),
    )
  }
}
