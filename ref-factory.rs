use std::rc::Rc;

fn main() {
  let _x = factory();
}

fn factory() -> Vec<Box<Trait>> {
  let big_struct = Rc::new(A::new());
  vec![
    Box::new(Wrap { _a: big_struct.clone() }),
    Box::new(Wrap { _a: big_struct.clone() }),
  ]
}

trait Trait {}

struct Wrap {
  _a: Rc<A>,
}

impl Trait for Wrap {}

struct A;

impl A {
  fn new() -> A { A }
}
