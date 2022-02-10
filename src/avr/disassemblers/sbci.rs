use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbci::Sbci;

impl Disassembler for Sbci {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBCI"),
      format!("R{}", self.d),
      format!("{}", self.k),
    )
  }
}
