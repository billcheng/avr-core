use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbis::Sbis;

impl Disassembler for Sbis {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBIS"),
      format!("{}", self.a),
      format!("{}", self.b),
    )
  }
}
