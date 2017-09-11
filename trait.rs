use std::ops::{Add, Mul};

fn main() {
  let a = Vector2 { x: 1, y: 2 };
  let b = Vector2 { x: 3, y: 4 };
  println!("{:?}", a.dot(b));
  println!("{:?}", a.sqr_len());
}

trait Vector<T>
    where Self: Sized + Copy {
  fn dot(self, Self) -> T;

  fn sqr_len(self) -> T {
    self.dot(self)
  }
}

#[derive(Debug, Clone, Copy)]
struct Vector2<T> {
  x: T,
  y: T,
}

impl<T> Vector<T> for Vector2<T>
    where T: Copy + Add<Output = T> + Mul<Output = T> {
  fn dot(self, others: Vector2<T>) -> T {
    self.x * others.x + self.y * others.y
  }
}

