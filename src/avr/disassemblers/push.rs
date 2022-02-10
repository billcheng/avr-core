use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::push::Push;

impl Disassembler for Push {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("PUSH"),
      format!("R{}", self.d),
      None,
    )
  }
}
