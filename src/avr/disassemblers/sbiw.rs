use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbiw::Sbiw;

impl Disassembler for Sbiw {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBIW"),
      Some(format!("R{}:R{}", self.d + 1, self.d)),
      Some(format!("{}", self.k)),
    )
  }
}
