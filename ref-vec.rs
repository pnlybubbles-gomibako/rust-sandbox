fn main() {
  let a = vec![A::new(3, 5), A::new(1, 10), A::new(2, 0)];
  println!("{:?}", a.len());
  let mut b: Vec<&A> = Vec::with_capacity(a.len());
  for v in &a {
    b.push(&v);
  }
  let mut c: Vec<&A> = (&a).iter().collect();
  // let mut b: Vec<&A> = vec![&a[0], &a[1], &a[2]];
  b.sort_by_key( |k| { k.a } );
  c.sort_by_key( |k| { k.b } );
  println!("{:?}", a);
  println!("{:?}", b[0]);
  println!("{:?}", c[0]);
}

#[derive(Debug)]
struct A {
  a: i32,
  b: i32,
}

impl A {
  fn new(a: i32, b: i32) -> A {
    A { a: a, b: b }
  }
}
