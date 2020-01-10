#[derive(Debug)]
enum Direction {
  Up(Point),
  Down(Point),
  Left(Point),
  Right(Point),
}

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug)]
enum Keys {
  UpKey(String),
  DownKey(String),
  LeftKey(String),
  RightKey(String),
}

impl Direction {
  fn match_direction(&self) -> Keys {
    // * -> Dereferences self
    match *self {
      Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
      Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
      Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
      Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
    }
  }
}

impl Keys {
  fn destruct(&self) -> &String {
    match *self {
      Keys::UpKey(ref s) => s,
      Keys::DownKey(ref s) => s,
      Keys::LeftKey(ref s) => s,
      Keys::RightKey(ref s) => s,
    }
  }
}

// Options -> Basic enum type
// enum Option<T> {
//   Some(T),
//   None,
// }

// Result lets you check errors
enum Result<T, E> {
  Ok(T),
  Err(E),
}

pub fn run () {
  let u = Direction::Up(Point { x: 0, y: 1 });
  let k = u.match_direction();
  let s = k.destruct();

  println!("{:?}", k);
  println!("{}", s);

  // using ref
  let u = 10;
  // the next two statements are equivlent
  let v = &u;
  let ref z = u;

  if z == v {
    println!("They are equal");
  }
}