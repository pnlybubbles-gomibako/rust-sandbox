use std::ops::{Neg, Add, Sub, Mul, Div};

fn main() {
  let a = Vector2 { x: 1.0, y: 2.0 };
  let b = Vector2 { x: 3.0, y: 4.0 };
  println!("{:?}", a.dot(b));
  println!("{:?}", a.sqr_len());
  println!("{:?}", a.len());
  println!("{:?}", a.norm());
  println!("{:?}", a + b);
  println!("{:?}", a * b);
}

trait Vector<T> : Neg<Output = Self> + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Mul<T, Output = Self>
    where Self: Sized + Copy {
  fn dot(self, Self) -> T;

  fn sqr_len(self) -> T {
    self.dot(self)
  }
}

trait VectorFloat<T> : Vector<T> + Div<T, Output = Self> {
  fn len(self) -> T;

  fn norm(self) -> Self {
    self / self.len()
  }
}

#[derive(Debug, Clone, Copy)]
struct Vector2<T> {
  x: T,
  y: T,
}

impl<T> Vector2<T>
    where T: Copy {
  fn new(x: T, y: T) -> Vector2<T> {
    Vector2 { x: x, y: y }
  }

  fn to_array(self) -> [T; 2] {
    [self.x, self.y]
  }
}

impl<T> Neg for Vector2<T>
    where T: Copy + Neg<Output = T> {
  type Output = Vector2<T>;

  fn neg(self) -> Vector2<T> {
    Vector2 { x: -self.x, y: -self.y }
  }
}

impl<T> Add for Vector2<T>
    where T: Copy + Add<Output = T> {
  type Output = Vector2<T>;

  fn add(self, others: Vector2<T>) -> Vector2<T> {
    Vector2 { x: self.x + others.x, y: self.y + others.y }
  }
}

impl<T> Sub for Vector2<T>
    where T: Copy + Sub<Output = T> {
  type Output = Vector2<T>;

  fn sub(self, others: Vector2<T>) -> Vector2<T> {
    Vector2 { x: self.x - others.x, y: self.y - others.y }
  }
}

impl<T> Mul<Vector2<T>> for Vector2<T>
    where T: Copy + Mul<Output = T> {
  type Output = Vector2<T>;

  fn mul(self, others: Vector2<T>) -> Vector2<T> {
    Vector2 { x: self.x * others.x, y: self.y * others.y }
  }
}

impl<T> Mul<T> for Vector2<T>
    where T: Copy + Mul<Output = T> {
  type Output = Vector2<T>;

  fn mul(self, others: T) -> Vector2<T> {
    Vector2 { x: self.x * others, y: self.y * others }
  }
}

impl<T> Div<Vector2<T>> for Vector2<T>
    where T: Copy + Div<Output = T> {
  type Output = Vector2<T>;

  fn div(self, others: Vector2<T>) -> Vector2<T> {
    Vector2 { x: self.x / others.x, y: self.y / others.y }
  }
}

impl<T> Div<T> for Vector2<T>
    where T: Copy + Div<Output = T> {
  type Output = Vector2<T>;

  fn div(self, others: T) -> Vector2<T> {
    Vector2 { x: self.x / others, y: self.y / others }
  }
}

impl<T> Vector<T> for Vector2<T>
    where T: Copy + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> {

  fn dot(self, others: Vector2<T>) -> T {
    self.x * others.x + self.y * others.y
  }
}

impl VectorFloat<f64> for Vector2<f64> {
  fn len(self) -> f64{
    self.sqr_len().sqrt()
  }
}

