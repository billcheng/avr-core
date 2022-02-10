use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbic::Sbic;

impl Disassembler for Sbic {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBIC"),
      format!("{}", self.a),
      format!("{}", self.b),
    )
  }
}
