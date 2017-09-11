use std::sync::Arc;
use std::thread;

fn main() {
  let a = Arc::new(A {});
  let b = Arc::new(B {});
  let c1 = Wrap { data: a.clone() };
  let c2 = Wrap { data: b.clone() };
  let c3 = Wrap { data: a.clone() };
  let v = Arc::new(Bundle {
    list: vec![c1, c2, c3],
  });
  for _ in 1..5 {
    let v = v.clone();
    thread::spawn(move || {
      let x = v;
      let y = &x.list[0].data;
      y.nyan();
    });
  }
}

struct Bundle {
  list: Vec<Wrap>,
}

struct Wrap {
  data: Arc<Base + Send + Sync>,
}

trait Base {
  fn nyan(&self);
}

#[derive(Debug)]
struct A {}

impl Base for A {
  fn nyan(&self) {}
}

#[derive(Debug)]
struct B {}

impl Base for B {
  fn nyan(&self) {}
}
