use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbi::Sbi;

impl Disassembler for Sbi {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBI"),
      Some(format!("{}", self.a)),
      Some(format!("{}", self.b)),
    )
  }
}
