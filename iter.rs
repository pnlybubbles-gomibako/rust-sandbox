fn main() {
  let a = [0u8, 1, 2, 3, 4];
  for v in &a {
    println!("{:?}", v);
  }
  for v in a.iter() {
    println!("{:?}", v);
  }
  for v in a.into_iter() {
    println!("{:?}", v);
  }
}
