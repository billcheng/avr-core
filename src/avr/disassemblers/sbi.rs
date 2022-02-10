use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbi::Sbi;

impl Disassembler for Sbi {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBI"),
      format!("{}", self.a),
      format!("{}", self.b),
    )
  }
}
