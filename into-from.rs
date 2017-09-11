fn main() {
  let a = Wrap { x: 10i32 };
  let b: Wrap<f64> = a.into();
  println!("{:?}", b);
}

#[derive(Debug, Clone, Copy)]
struct Wrap<T> {
  x: T
}

impl<T, S> From<Wrap<S>> for Wrap<T> where S: From<T> {
  fn from(v: Wrap<S>) -> Self {
    Wrap { x: S::from(v.x) }
  }
}
