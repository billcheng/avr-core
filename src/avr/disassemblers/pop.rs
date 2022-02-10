use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::pop::Pop;

impl Disassembler for Pop {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("POP"),
      format!("R{}", self.d),
      None,
    )
  }
}
