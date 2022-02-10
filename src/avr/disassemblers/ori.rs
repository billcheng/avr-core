use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ori::Ori;

impl Disassembler for Ori {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ORI"),
      format!("R{}", self.d),
      format!("{}", self.k),
    )
  }
}
