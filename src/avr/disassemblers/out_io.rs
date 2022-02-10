use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::out_io::Out;

impl Disassembler for Out {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("OUT"),
      Some(format!("{}", self.a)),
      Some(format!("R{}", self.r)),
    )
  }
}
