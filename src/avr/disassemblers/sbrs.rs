use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbrs::Sbrs;

impl Disassembler for Sbrs {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBRS"),
      format!("R{}", self.r),
      format!("{}", self.b),
    )
  }
}
