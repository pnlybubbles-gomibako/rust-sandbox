fn main() {
  let mut a: Data = Default::default();
  let mut i = 0;
  a.each_mut( |v| {
    i += 1;
    *v += i;
  } );
  println!("{:?}", a);
  a.apply(2, |v| {
    *v += 10;
  } );
  println!("{:?}", a);
}

#[derive(Debug, Default)]
struct Data {
  data: [u8; 5],
}

impl Data {
  fn set(&mut self, i: usize, v: u8) {
    self.data[i] = v
  }

  fn apply<F>(&mut self, i: usize, mut f: F) where F: FnMut(&mut u8) {
    f(&mut self.data[i])
  }

  fn each<F>(&self, mut f: F) where F: FnMut(&u8) {
    for v in self.data.iter() {
      f(v)
    }
  }

  fn each_mut<F>(&mut self, mut f: F) where F: FnMut(&mut u8) {
    for v in self.data.iter_mut() {
      f(v)
    }
  }
}
