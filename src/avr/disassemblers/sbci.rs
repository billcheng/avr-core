use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbci::Sbci;

impl Disassembler for Sbci {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBCI"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.k)),
    )
  }
}
