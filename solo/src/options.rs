
// Options -> Basic enum type
// enum Option<T> {
//   Some(T),
//   None,
// }

pub fn run () {
  option_match();
}

fn option_match() {
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  println!("{:?}, {:?}, {:?}", five, six, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
      None => None,
      Some(i) => Some(i + 1),
  }
}