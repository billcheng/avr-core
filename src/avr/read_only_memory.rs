pub trait ReadOnlyMemory {
  fn read(&self, address: u32) -> u16;
}
