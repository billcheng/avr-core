pub trait RandomAccessMemory {
  fn read(&self, address: u32) -> u8;
  fn write(&mut self, address: u32, value: u8);
}
