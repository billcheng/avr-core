use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::or::Or;

impl Disassembler for Or {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("OR"),
      format!("R{}", self.d),
      format!("R{}", self.r),
    )
  }
}
