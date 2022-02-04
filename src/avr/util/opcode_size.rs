pub trait OpcodeSize {
  fn get_size(&self, opcode: u16) -> usize;
}

pub struct Opcode {}

impl Opcode {
  pub fn new() -> Self {
    Self {}
  }
}

impl OpcodeSize for Opcode {
  fn get_size(&self, opcode: u16) -> usize {
    let is_call = opcode & 0b1111_1110_0000_1110 == 0b1001_0100_0000_1110;
    let is_jmp = opcode & 0b1111_1110_0000_1110 == 0b1001_0100_0000_1100;
    let is_lds = opcode & 0b1111_1110_0000_1111 == 0b1001_0000_0000_0000;
    let is_sts = opcode & 0b1111_1110_0000_1111 == 0b1001_0010_0000_0000;

    match is_call || is_jmp || is_lds || is_sts {
      true => 2,
      false => 1,
    }
  }
}
