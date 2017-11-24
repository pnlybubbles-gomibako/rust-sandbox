fn main() {
  let _x = factory();
}

fn factory() -> Vec<Box<Trait>> {
  let big_struct = A::new();
  vec![
    Box::new(Wrap { _a: &big_struct }),
    Box::new(Wrap { _a: &big_struct }),
  ]
}

trait Trait {}

struct Wrap<'a> {
  _a: &'a A,
}

impl<'a> Trait for Wrap<'a> {}

struct A;

impl A {
  fn new() -> A { A }
}
