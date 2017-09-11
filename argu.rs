fn main() {
  let a = Test { value: 1 };
  println!("{:?}", a);
  let a_ = normal(a); // a is moved
  println!("{:?}", a_);

  let b = Test { value: 1 };
  let b_ = reference(&b);
  println!("{:?}", b); // b is borrowed
  println!("{:?}", b_);

  let mut c = Test { value: 1 };
  println!("{:?}", c);
  c = mutable(c); // c is moved and moved to return
  println!("{:?}", c);

  let mut d = Test { value: 1 };
  println!("{:?}", d);
  mutalbe_reference(&mut d); // d is borrowed
  println!("{:?}", d);
}

#[derive(Debug)]
struct Test {
  value: i32,
}

fn normal(a: Test) -> i32 {
  a.value + 1
}

fn reference(a: &Test) -> i32 {
  a.value + 1
}

fn mutable(mut a: Test) -> Test {
  a.value = a.value + 1;
  a
}

fn mutalbe_reference (a: &mut Test) {
  a.value = a.value + 1
}

