#![feature(box_syntax)]

fn main() {
  let ab = HasAB;
  use_a(&ab);
  use_b(&ab);
  use_a_box(box ab);
  use_b_box(box ab);
}

trait A {}
trait B {}

#[derive(Copy, Clone)]
struct HasAB;

impl A for HasAB {}
impl B for HasAB {}

fn use_a(value: &A) {}
fn use_b(value: &B) {}

fn use_a_box(value: Box<A>) {}
fn use_b_box(value: Box<B>) {}
