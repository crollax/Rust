use std::fmt;

#[derive(Debug)] // Trait
struct Object {
  width: u32,
  height: u32,
}

// Methods
impl Object { 
  fn area(&self) -> u32 {
    self.width * self.height
  } 

  fn show(&self) {
    println!("{}x{} with area: {}", self.width, self.height, self.area());
  }
}

// Related Funcs
  // can not be called without calling namespace
  // ex. let obj = Object::new(57,)
impl Object {
  fn new(width: u32, height: u32) -> Object {
    Object {
      width,
      height,
    }
  }
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.width, self.height)
  }
}

pub fn run() {
  let o = Object {
    width: 35,
    height: 55,
  };

  let obj = Object::new(57, 83);

  o.show();
  obj.show();

  println!("{}", o);
  println!("{}", obj);
}