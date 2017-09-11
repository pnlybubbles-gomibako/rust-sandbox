fn main() {
  let a = Test { value: 1 };
  let a_c = |v| { a.value + v }; // a is borrowed
  let a_ = normal(1, a_c); // a_c is moved
  println!("{:?}", a);
  println!("{:?}", a_);

  let b = Test { value: 1 };
  println!("{:?}", b);
  let b_c = move |v| { b.value + v }; // b is moved
  let b_ = normal(1, b_c); // b_c is moved
  println!("{:?}", b_);

  let c = Test { value: 1 };
  let c_c = |v| { c.value + v }; // c is borrowed
  let c_ = normal(1, &c_c); // c_c is borrowed
  let c__ = normal(2, &c_c); // c_c is borrowed
  println!("{:?}", c);
  println!("{:?}", c_);
  println!("{:?}", c__);

  let d = Test { value: 1 };
  println!("{:?}", d);
  let d_c = move |v| { d.value + v }; // d is moved
  let d_ = normal(1, &d_c); // d_c is borrowed
  let d__ = normal(2, &d_c); // d_c is borrowed
  println!("{:?}", d_);
  println!("{:?}", d__);

  let e = Test { value: 1 };
  let e_c = |v| { e.value + v }; // e is borrowed
  let e_ = reference(1, &e_c); // e_c is borrowed
  let e__ = reference(2, &e_c); // e_c is borrowed
  println!("{:?}", e);
  println!("{:?}", e_);
  println!("{:?}", e__);

  let f = Test { value: 1 };
  println!("{:?}", f);
  let f_c = move |v| { f.value + v }; // f is moved
  let f_ = reference(1, &f_c); // f_c is borrowed
  let f__ = reference(2, &f_c); // f_c is borrowed
  println!("{:?}", f_);
  println!("{:?}", f__);

  let mut g = Test { value: 1 };
  println!("{:?}", g);
  mutable(1, |v| { // g is mutable borrowed
    g.value += v
  }); // g is returned
  println!("{:?}", g);

  let mut h = Test { value: 1 };
  println!("{:?}", h);
  {
    let h_c = |v| { h.value += v }; // h is mutable borrowed
    // cannot borrow h here as it is borrowed as mutable
    mutable(1, h_c); // h_c is moved
  } // h_c is dead, h is returned
  println!("{:?}", h);

  let mut i = Test { value: 1 };
  println!("{:?}", i);
  {
    let mut i_c = |v| { i.value += v }; // i is mutable borrowed
    // cannot borrow i here as it is borrowed as mutable
    mutable_reference(1, &mut i_c); // i_c is moved
    mutable_reference(2, &mut i_c); // i_c is moved
  } // i_c is dead, i is returned
  println!("{:?}", i);
}

#[derive(Debug)]
struct Test {
  value: i32,
}

fn normal<F>(n: i32, f: F) -> i32 where F: Fn(i32) -> i32 {
  f(n)
}

fn reference<F>(n: i32, f: &F) -> i32 where F: Fn(i32) -> i32 {
  f(n)
}

fn mutable<F>(n: i32, mut f: F) where F: FnMut(i32) {
  f(n)
}

fn mutable_reference<F>(n: i32, f: &mut F) where F: FnMut(i32) {
  f(n)
}

// fn normal<F>(a: Test, f: F) -> i32 where F: Fn(Test) -> i32 {
//   f(a)
// }
