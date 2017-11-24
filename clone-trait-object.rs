fn main() {
  let d = desc();
  for x in d.iter() {
    println!("{}", x.value.get())
  }
}

fn desc<'a>() -> Vec<Wrap<'a>> {
  let a: &'a Box<Trait + Send + Sync> = &Box::new(A::new(1));
  let b: &'a Box<Trait + Send + Sync> = &Box::new(A::new(2));
  vec![
    Wrap::new(&a),
    Wrap::new(&a),
    Wrap::new(&b),
  ]
}

struct Wrap<'a> {
  value: &'a Box<Trait + Send + Sync>,
}

impl<'a> Wrap<'a> {
  fn new(value: &Box<Trait + Send + Sync>) -> Wrap {
    Wrap { value: value }
  }
}

trait Trait {
  fn get(&self) -> u8;
}

struct A { v: u8 }

impl A {
  fn new(v: u8) -> A { A { v: v } }
}

impl Trait for A {
  fn get(&self) -> u8 { self.v }
}
