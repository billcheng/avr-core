use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::andi::Andi;

impl Disassembler for Andi {
  fn disassemble(
    &self,
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
