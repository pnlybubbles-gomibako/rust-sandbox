fn main() {
  let a: Vec<A> = vec![A {v: 3, u: 2}, A {v: 1, u: 3}, A {v: 2, u: 1}];
  let mut b: Vec<&A> = a.iter().collect();
  let mut c: Vec<(usize, &A)> = a.iter().enumerate().collect();
  b.sort_by_key( |k| k.v );
  c.sort_by_key( |&(_, k)| k.u );
  println!("{:?}", a);
  println!("{:?}", b);
  println!("{:?}", c);
}

#[derive(Debug)]
struct A {
  v: i32,
  u: i32,
}
