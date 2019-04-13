#![feature(box_syntax)]

fn main() {
  let mut vl = vec![
    V { l: 4, r: 8 },
    V { l: 0, r: 2 },
    V { l: 7, r: 9 },
    V { l: 3, r: 5 },
  ];
  vl.sort_by_key( |v| (v.l + v.r) / 2 );
  let tree = new(vl);
}

fn new(vl: Vec<V>) -> Box<Tree> {
  let list = vl.into_iter().map( |v|
    box Leaf {
      value: (v.l + v.r) / 2,
      cont: &v,
    }
  ).collect::<Vec<_>>();
  construct(&list)
}

fn construct<'a>(list: &'a [Box<Leaf>]) -> Box<Tree + 'a> {
  let len = list.len();
  if len == 1 {
    list[0]
  } else {
    let max = list.iter().fold(list[0].cont.r, |p, c| p.max(c.cont.r) );
    let min = list.iter().fold(list[0].cont.l, |p, c| p.min(c.cont.l) );
    let p = len / 2;
    box Node {
      value: (max + min) / 2,
      children: [
        construct(&list[0..p]),
        construct(&list[p..len]),
      ]
    }
  }
}

#[derive(Debug)]
struct V {
  l: u8,
  r: u8,
}

trait Tree {
}

struct Node<'a> {
  value: u8,
  children: [Box<Tree + 'a>; 2],
}

impl<'a> Tree for Node<'a> {
}

struct Leaf<'a> {
  value: u8,
  cont: &'a V,
}

impl<'a> Tree for Leaf<'a> {
}
