fn main() {
  let origin = vec![A { v: 5 }, A { v: 2 }, A { v: 7 }, A { v: 1 }, A { v: 3 }];
  let mut copy = dup(&origin);
  copy.sort_by_key( |v| v.v );
  for v in &origin {
    print!("{} ", v.v)
  }
  println!("");
  for v in &copy {
    print!("{} ", v.v)
  }
  println!("");
  {
    let im_copy = copy;
    let filtered = im_copy.iter().filter( |v| v.v > 2 ).collect::<Vec<_>>();
    for v in &filtered {
      print!("{} ", v.v)
    }
    println!("");
  }
}

fn dup(x: &Vec<A>) -> Vec<&A> {
  x.iter().collect()
}

struct A { v: u8 }
