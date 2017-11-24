#![feature(box_syntax)]

use std::rc::Rc;

fn main() {
  let origin = vec![A { v: 5 }, A { v: 2 }, A { v: 7 }, A { v: 1 }, A { v: 3 }];
  let _t = construct(origin.into_iter().map( |v| Rc::new(v) ).collect());
}

fn construct(list: Vec<Rc<A>>) -> Box<Tree> {
  if list.len() == 1 {
    box Leaf {
      value: list[0].clone(),
    }
  } else {
    let child = construct(list[1..].iter().cloned().collect());
    box Node {
      value: list[0].clone(),
      child: child,
    }
  }
}

struct A { v: u8 }

struct Node {
  value: Rc<A>,
  child: Box<Tree>,
}

struct Leaf {
  value: Rc<A>,
}

trait Tree {
}

impl Tree for Node {}
impl Tree for Leaf {}
