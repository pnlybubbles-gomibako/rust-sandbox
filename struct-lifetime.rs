#![feature(box_syntax)]

fn main() {
  let v = vec![
    V { value: 0 },
    V { value: 5 },
    V { value: 3 },
  ];
  let a = construct(&v[..]);
  println!("{}", a.get().value);
}

fn construct<'a>(v: &'a [V]) -> Box<AB + 'a> {
  if v.len() == 1 {
    box A { cont: &v[0] }
  } else {
    box B { succ: construct(&v[1..]) }
  }
}

struct V {
  value: u8,
}

struct A<'a> {
  cont: &'a V,
}

struct B<'a> {
  succ: Box<AB + 'a>,
}

trait AB {
  fn get(&self) -> &V;
}

impl<'a> AB for A<'a> {
  fn get(&self) -> &V {
    self.cont
  }
}

impl<'a> AB for B<'a> {
  fn get(&self) -> &V {
    self.succ.get()
  }
}
