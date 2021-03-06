use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::adc::Adc;

impl Disassembler for Adc {
  fn disassemble(&self, _address: u32) -> (String, Option<String>, Option<String>) {
    (
      String::from("ADC"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
