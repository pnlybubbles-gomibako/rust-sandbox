fn main() {
  let a = vec![A {v: 0}, A {v: 1}];
  let b = new(&a);
  println!("{}", a[0].v);
}

fn new(v: &Vec<A>) -> Vec<Wrap> {
  v.into_iter().map( |v|
    Wrap { value: v }
  ).collect::<Vec<_>>()
}

struct A {
  v: u8,
}

trait Trait {}

struct Wrap<'a> {
  value: &'a A,
}

impl Trait for Wrap {}
