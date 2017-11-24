#[derive(Debug)]
struct A;

impl A {
  fn new() -> A { A }
}

fn main() {
  // let _a = vec![ &A::new() ];
  let _b = vec![ Box::new(A::new()) ];
}
