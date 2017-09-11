fn main() {
  let mut a = [1., 2., 3., 4., 5.];
  let b = [&[1., 2.][..], &[3., 4.][..]];
  test(&mut a);
  test2(&b[..]);
  println!("{:?}", a);
  println!("{:?}", b);
}

fn test(s: &mut [f64]) {
  println!("{:?}", s.len());
  for i in 0..s.len() {
    s[i] = s[i] + 1.;
  }
}

fn test2(s: &[&[f64]]) {
  for i in 0..s.len() {
    for j in 0..s[0].len() {
      println!("{:?}", s[i][j]);
    }
  }
}

