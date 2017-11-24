fn main() {
  let _x = factory();
}

fn factory<'a>() -> (Vec<Wrap<'a>>, A) {
  let big_struct = A;
  (vec![
    Wrap { _n: 1, _a: &big_struct },
    Wrap { _n: 2, _a: &big_struct },
  ], big_struct)
}

// fn factory1<'a>(big_struct: &'a A) -> Vec<Wrap<'a>> {
//   vec![
//     Wrap { _n: 1, _a: &big_struct },
//     Wrap { _n: 2, _a: &big_struct },
//   ]
// }

struct A;

struct Wrap<'a> { _n: u8, _a: &'a A }
