fn main() {
  let mut a: Data = Default::default();
  println!("{:?}", a);
  a.each_mut( |v| {
    *v = *v + 1
  } );
  println!("{:?}", a);
  let mut b = 0u8;
  a.each( |v| {
    b += *v
  } );
  println!("{:?}", b);
  a.each_with_index( |v, x, y| {
    b += *v + x as u8 + y as u8
  } );
  println!("{:?}", b);
}

#[derive(Default, Debug)]
struct Data {
  data: [[u8; 3]; 3],
}

impl Data {
  fn each<F>(&self, mut f: F) where F: FnMut(&u8) {
    for row in self.data.iter() {
      for v in row.iter() {
        f(v)
      }
    }
  }
  fn each_with_index<F>(&self, mut f: F) where F: FnMut(&u8, usize, usize) {
    for (x, col) in self.data.iter().enumerate() {
      for (y, v) in col.iter().enumerate() {
        f(v, x, y)
      }
    }
  }
  fn each_mut<F>(&mut self, mut f: F) where F: FnMut(&mut u8) {
    for col in self.data.iter_mut() {
      for v in col.iter_mut() {
        f(v)
      }
    }
  }
}
