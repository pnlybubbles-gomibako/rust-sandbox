const WIDTH: usize = 2;
const HEIGHT: usize = 2;

#[derive(Debug, Default)]
struct Output<T> {
  data: [[T; HEIGHT]; WIDTH],
}
impl <T> Output<T> {
  fn each<F: Fn(usize, usize, &T)>(&self, f: F) {
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        f(x, y, &self.data[x][y]);
      }
    }
  }
}

fn main() {
  let mut output: Output<f64> = Output::default();
  output.each( |x, y, v| {
    println!("{:?}, {:?}: {:?}", x, y, v);
    // output.data[x][y] = (x + y) as f64;
  });
  println!("{:?}", output);
}
