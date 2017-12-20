#![feature(box_syntax)]

fn main() {
  let v: Vec<Box<AB>> = vec![
    box S,
    box T,
  ];
  use_ab(v[0]);
  use_a(v[1] as Box<A>);
}


fn use_ab(v: Box<AB>) {}
fn use_a(v: Box<A>) {}

trait A {}
trait B {}

trait AB: A + B {}

#[derive(Copy, Clone)]
struct S;
#[derive(Copy, Clone)]
struct T;

impl A for S {}
impl B for S {}
impl AB for S {}
impl A for T {}
impl B for T {}
impl AB for T {}
