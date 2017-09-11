fn main() {
  let mut a = Data::default();
  println!("{:?}", a);
  a.apply( |v| {
    v + 1
  } );
  println!("{:?}", a);

  let mut b = Data::default();
  println!("{:?}", b);
  b.each_mut( |v| {
    *v = *v + 1
  } );
  println!("{:?}", b);
}

#[derive(Debug, Default)]
struct Data {
  data: [u8; 5],
}

impl Data {
  fn apply<F>(&mut self, f: F) where F: Fn(u8) -> u8 {
    for v in self.data.iter_mut() {
      *v = f(*v)
    }
  }

  fn each_mut<F>(&mut self, f: F) where F: Fn(&mut u8) {
    for v in self.data.iter_mut() {
      f(v)
    }
  }
}
