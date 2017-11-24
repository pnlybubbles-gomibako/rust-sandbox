fn main() {
  let a: Vec<&Get> = vec![&A { v: 1 }, &B { v: 2 }];
  for x in a {
    println!("{}", x.get())
  }
}

trait Get {
  fn get(&self) -> u8;
}

struct A { v: u8 }

impl Get for A {
  fn get(&self) -> u8 {
    self.v
  }
}

struct B { v: u8 }

impl Get for B {
  fn get(&self) -> u8 {
    self.v
  }
}
