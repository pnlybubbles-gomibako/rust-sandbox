fn main() {
  let a = vec![None, Some(1u8), None, Some(2)];
  // let r: Option<u8> = a.into_iter().fold(None, |p, c|
  //   c.map( |d| p.map( |q| d + q.pow(2) ).unwrap_or(d)).or(p)
  // );
  // let r = a.into_iter().fold(None, |p, c|
  //   match c {
  //     None => p,
  //     Some(d) => p.map( |q| d + q.pow(2)).or(Some(d)),
  //   }
  // );
  // let r = a.into_iter().fold(None, |p, c|
  //   match c {
  //     None => p,
  //     Some(d) => match p {
  //       None => Some(d),
  //       Some(q) => Some(d + q.pow(2)),
  //     },
  //   }
  // );
  let r = a.iter().flat_map( |v| v ).min_by( |a, b| a.pow(2).cmp(&b.pow(2)) );
  println!("{:?}", r);
}
