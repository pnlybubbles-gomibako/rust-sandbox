fn main() {
  let a = vec![1, 2, 3, 4, 5, 6];
  let b = &a[0..3];
  let c = &a[3..];
  println!("{:?}", a);
  println!("{:?}", b);
  println!("{:?}", c);
}
