use std::sync::Arc;
use std::thread;

fn main() {
  let a = Box::new(Elm { value: 1 });
  let b = Box::new(Elm { value: 2 });
  let c = Arc::new(Elm { value: 3 });
  let rwa = RefWrap { content: &*a };
  let rwc = RefWrap { content: &*c };
  // let wac = WrapArc { content: c };
  // let wwa = RefWrapWrap { content: w };
  thread::spawn(move || {
    let x = rwc;
  });
}

struct RefWrapWrap<'a> {
  content: RefWrap<'a>,
}

impl<'a> RefWrapWrap<'a> {
  fn b(&self) {}
}

struct RefWrap<'a> {
  content: &'a A,
}

struct Wrap {
  content: Box<A>,
}

struct WrapArc {
  content: Arc<A>,
}

trait A : Sync {
  fn a(&self) -> i32;
}

#[derive(Debug)]
struct Elm {
  value: i32,
}

impl A for Elm {
  fn a(&self) -> i32 {
    self.value
  }
}

