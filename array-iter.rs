fn main() {
  let a = vec![B, B, B];
  let a1 = A {v: 3};
  let a2 = A {v: 1};
  let a3 = A {v: 2};
  let mut c = [a1, a2, a3];
  let a4 = A {v: 4};
  c[0] = a4;
  let b = c.iter().zip(a.iter()).max_by_key( |&(k, _)| k.v ).unwrap();
  println!("{:?}", a);
  println!("{:?}", b);
}

#[derive(Debug)]
struct A { v: i32 }
#[derive(Debug)]
struct B;
