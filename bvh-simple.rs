#![feature(box_syntax)]

fn main() {
  let mut list: Vec<Box<Shape>> = vec![
    box Triangle { x: 5, y: 0 },
    box Triangle { x: 1, y: 1 },
    box Triangle { x: 0, y: 2 },
    box Triangle { x: 2, y: 3 },
    box Triangle { x: 3, y: 4 },
    box Triangle { x: 8, y: 5 },
    box Triangle { x: 4, y: 6 },
    box Triangle { x: 7, y: 7 },
  ];
  let bvh = construct(&mut list, 0);
}

#[derive(Clone)]
struct AABB {
  x: i32,
  y: i32,
}

trait Shape {
  fn aabb(&self) -> AABB;
}

struct Triangle { x: i32, y: i32 }

impl Shape for Triangle {
  fn aabb(&self) -> AABB {
    AABB { x: self.x, y: self.y }
  }
}

struct Leaf<'a> {
  aabb: AABB,
  object: &'a Box<Shape>,
}

impl<'a> Shape for Leaf<'a> {
  fn aabb(&self) -> AABB {
    self.aabb.clone()
  }
}

struct Node<'a> {
  aabb: AABB,
  left: Box<Shape + 'a>,
  // right: Box<Shape + 'a>,
}

impl<'a> Shape for Node<'a> {
  fn aabb(&self) -> AABB {
    self.aabb.clone()
  }
}

fn construct<'a>(list: &'a mut [Box<Shape>], depth: usize) -> Box<Shape + 'a> {
  // if list.len() == 1 {
  //   return box Leaf {
  //     aabb: list[0].aabb(),
  //     object: &list[0],
  //   }
  // }
  // list.sort_by_key( |v|
  //   if depth % 2 == 0 { v.aabb().x } else { v.aabb().y }
  // );
  // let mid = list.len() / 2;
  // let (l, r) = list.split_at_mut(mid);
  // box Node {
  //   aabb: AABB { x: 0, y: 0 },
  //   left: construct(&mut l, depth + 1),
  //   right: construct(&mut r, depth + 1),
  // }
  if list.len() == 1 {
    box Leaf {
      aabb: AABB { x: 0, y: 0 },
      object: &list[0],
    }
  } else {
    let mid = list.len();
    // let (l, r) = list.split_at_mut(mid / 2);
    let mut l = &mut list[1..];
    box Node {
      aabb: AABB { x: 0, y: 0 },
      left: construct(&mut l, depth + 1),
      // right: construct(&mut r, depth + 1),
    }
  }
}
