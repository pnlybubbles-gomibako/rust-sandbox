fn main() {
  let a = Big { value: 6 };
  let b = Small { value: 3 };
  let ac = Testable { content: Box::new(a) };
  let bc = Testable { content: Box::new(b) };
  println!("{:?}", ac.inverse());
  println!("{:?}", bc.inverse());
}

struct Testable {
  content: Box<Test>,
}

impl Testable {
  fn inverse(&self) -> bool {
    !self.content.test()
  }
}

trait Test {
  fn test(&self) -> bool;
}

struct Big {
  value: i32,
}

impl Test for Big {
  fn test(&self) -> bool {
    self.value > 10
  }
}

struct Small {
  value: i32,
}

impl Test for Small {
  fn test(&self) -> bool {
    self.value < 5
  }
}
