#![feature(box_syntax)]

fn main() {
  // let box_cons_struct = Big::box_cons(); // SEGV
  let box_syntax_struct = Big::box_syntax(); // OK
  // let box_cons = Box::new([[[0f64; 3]; HEIGHT]; WIDTH]); // SEGV
  let box_syntax = box [[[0f64; 3]; HEIGHT]; WIDTH]; // OK
}

const HEIGHT: usize = 2048;
const WIDTH: usize = 2048;

struct Big {
  data: Box<[[[f64; 3]; HEIGHT]; WIDTH]>,
}

impl Big {
  fn box_cons() -> Big {
    Big {
      data: Box::new([[[0f64; 3]; HEIGHT]; WIDTH]),
    }
  }

  fn box_syntax() -> Big {
    Big {
      data: box [[[0f64; 3]; HEIGHT]; WIDTH],
    }
  }
}
