use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::mov::Mov;

impl Disassembler for Mov {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("MOV"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
