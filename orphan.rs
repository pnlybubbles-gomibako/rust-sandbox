use std::ops::Index;

fn main() {
  let a = Struct { v: 1 };
  println!("{}", a[2])
}

trait Trait {
  fn get(&self) -> u8;
}

struct Struct { v: u8 }

impl Trait for Struct {
  fn get(&self) -> u8 { self.v }
}

impl<T> Index<u8> for T where T: Trait {
  type Output = u8;

  fn index(&self, i: u8) -> u8 {
    self.get() + i
  }
}
