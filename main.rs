mod vector;

use vector::Vector;

fn main () {
  let a = Vector::new(1., 1., 1.);
  let b = Vector::new(1., 0., 2.);
  println!("{}", a.dot(b));
}
