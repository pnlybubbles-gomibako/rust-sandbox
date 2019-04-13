fn main() {
  let mut a = vec![
    C { v: 1 },
    C { v: 4 },
    C { v: 3 },
    C { v: 2 },
  ];
  test(&mut a);
}

#[derive(Debug)]
struct C {
  v: i32,
}

fn test(v: &mut [C]) -> &C {
  if v.len() == 1 {
    &v[0]
  } else {
    let mid = v.len() / 2;
    test(&mut v[0..mid]);
    let right = test(&mut v[mid..]);
    left
  }
}
