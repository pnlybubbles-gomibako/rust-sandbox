fn main() {
  let a: [f64; 3] = [1., 2., 3.];
  f(to_int(&a));
}

fn to_int(a: &[f64]) -> [u8; 3] {
  let mut c = [0u8; 3];
  for (&x, y) in a.iter().zip(c.iter_mut()) {
    *y = x as u8;
  }
  return c;
}

fn f(x: [u8; 3]) {
  println!("{:?}", x);
}
