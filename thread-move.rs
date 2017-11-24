#![feature(box_syntax)]
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

trait Get {
  fn get(&self) -> u8;
}

struct C {
  v: u8,
  a: A,
}

impl Get for C {
  fn get(&self) -> u8 {
    self.v
  }
}

struct A;

impl A {
  fn new() -> A { A }
}

fn main() {
  let a: Arc<Vec<Box<Get + Sync + Send>>> = Arc::new(vec![
    box C { v: 1, a: A::new() },
    box C { v: 2, a: A::new() },
    box C { v: 3, a: A::new() },
  ]);
  // let b: Arc<Vec<Box<Get + Sync + Send>>> = Arc::new(vec![
  //   Box::new(C { v: 4 }),
  //   Box::new(C { v: 7 }),
  //   Box::new(C { v: 5 }),
  // ]);
  let (tx, rx): (Sender<(usize, u8)>, Receiver<(usize, u8)>) = channel();
  for i in 0..3 {
    let tx = tx.clone();
    let a = a.clone();
    thread::spawn(move || {
      tx.send((i, op(&a, &a, i))).unwrap()
    });
  }
  let mut v = vec![0u8; 3];
  for _ in 0..3 {
    let (i, x) = rx.recv().unwrap();
    v[i] = x
  }
  println!("{:?}", v)
}

fn op<T: Get + ?Sized>(a: &Vec<Box<T>>, b: &Vec<Box<T>>, i: usize) -> u8 {
  a[i].get() + b[i].get()
}
