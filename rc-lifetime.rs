use std::sync::Arc;
use std::thread;

fn main() {
  let a = 1;
  // let b = Arc::new(&a);
  let b = &a;
  thread::spawn(move || {
    *b
  });
}

// fn main() {
//   let a = Init::new(1);
//   let b = Arc::new(a.init());
//   let (tx, rx) = mpsc::channel();
//   for v in 0..3 {
//     let tx = tx.clone();
//     let b = b.clone();
//     thread::spawn(move || {
//       tx.send(b.get() + v).unwrap();
//     });
//   }
//   let l = (0..3).map( |_| rx.recv().unwrap() ).collect::<Vec<_>>();
//   println!("{:?}", l);
// }

// struct Init {
//   v: Box<usize>,
// }

// impl Init {
//   fn new(v: usize) -> Init {
//     Init {
//       v: Box::new(v),
//     }
//   }

//   fn init<'a>(&'a self) -> A<'a> {
//     A { a: &self.v }
//   }
// }

// struct A<'a> {
//   a: &'a Box<usize>
// }

// impl<'a> A<'a> {
//   fn get(&self) -> usize {
//     **self.a
//   }
// }
