// What if you want to match multiple arms?

//If let or While let
  // Pattern matching
pub fn run_2 () {
  let mut s = Some(0);

  if let Some(i) = s {
    println!("i: {}", i);
  }

  while let Some(i) = s {
    if i > 19 { 
      println!("Quit");
      s = None;
    } else {
      println!("{}", i);
      s = Some(i + 2);
    }
  }
}

pub fn run () {
  // clone_match();
  check_shapes();
}

// Matching on values with no ownership
  // sets value of p to n 
fn clone_match () {
  let p = 0;
  let n = match p {
    n @ 1 ..= 12 => n,
    n @ 13 ..= 19 => n,
    _ => 0,
  };
  println!("n: {}", n)
}

fn guard_match () {
  let pair = (5, -5);
  match pair {
    (x, y) if x == y => println!("Equal"),
    (x, y) if x + y == 0 => println!("Equal Zero"),
    (x, _) if x % 2 == 0 => println!("X is even"),
    _ => println!("No match"),
  }
}

fn touple_match () {
  let pair = (0, -2);
  // Matches on the non var item
    // Binds the var to the other param (destructures the touple)
  match pair {
    (0, y) => println!("y: {}", y),
    (x, 0) => println!("x: {}", x),
     _     => println!("No match"),
  }
}

fn match1 () {
  let x = 10;
  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
  }
}

fn match2 () {
  let n = 15;
  match n {
    1 => println!("One!"),
    // Conditional check
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13 ..= 19 => println!("A teen"),
    _ => println!("Ain't special"),
  }
}

enum Shape {
  Rectangle {width: u32, height: u32},
  Square(u32),
  Circle(f64),
}

impl Shape{
  fn area(&self) -> f64 {
    match *self {
      Shape::Rectangle {width, height} => (width * height) as f64,
      Shape::Square(ref s) => (s * s) as f64,
      Shape::Circle(ref r) => 3.14 * (r * r),
    }
  }
}

fn check_shapes () {
  let r = Shape::Rectangle{width: 10, height: 70};
  let s = Shape::Square(10);
  let c = Shape::Circle(4.5);

  let ar = r.area();
  println!("{}", ar);

  let aq = s.area();
  println!("{}", aq);

  let ac = c.area();
  println!("{}", ac);
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

// Using a Result enum
use std::fs::File;
fn open_file() {
  let f = File::open("test.txt");
  let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error);
    }
  }
}