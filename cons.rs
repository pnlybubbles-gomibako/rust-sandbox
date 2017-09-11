fn main() {
  Cons {
    a: 10,
    .. Cons::default()
  }
}

struct Cons {
  a: u8,
  b: u8,
  c: u8,
}

impl Cons {
  fn new() {
  }
}
