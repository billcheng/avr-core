use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::out_io::Out;

impl Disassembler for Out {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("OUT"),
      format!("{}", self.a),
      format!("R{}", self.r),
    )
  }
}
