fn main() {
  for i in 1..100 {
    println!("{}", fizbuz(i).unwrap_or(&*i.to_string()));
  }
}
fn fizbuz(n: i32) -> Option<&'static str> {
  if n % 15 == 0 {
    Some("FizzBuzz")
  }else if n % 5 == 0 {
    Some("Buzz")
  }else if n % 3 == 0 {
    Some("Fizz")
  }else{
    None
  }
}
