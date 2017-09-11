fn main() {
  let b;
  {
    let a = [1u8; 3];
    b = copy(a);
  }
  println!("{:?}", b);
}

fn copy(a: [u8; 3]) -> [u8; 3] {
  let mut b = [1u8; 3];
  for (x, y) in a.iter().zip(b.iter_mut()) {
    *y = *x;
  }
  return b;
}
