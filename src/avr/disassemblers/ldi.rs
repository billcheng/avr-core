use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ldi::Ldi;

impl Disassembler for Ldi {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LDI"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.k)),
    )
  }
}
