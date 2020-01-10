trait Shape { 
  fn area(&self) -> u32;
}

struct Rectangle {
  x: u32,
  y: u32,
}

struct Circle {
  radius: f64;
}

impl Shape for Rectangle {
  fn area(&self) -> u32 {
    self.x * self.y
  }
}

impl Shape for Circle {
  fn area(&self) -> u32 {
    (3.141 * self.radius * self.radius) as u32
  }
}

/////////////////////////////////////////////////////////////////////////
// Using generics
/////////////////////////////////////////////////////////////////////////

use std::ops::Mul;
trait Shape_2<T> {
  fn area(&self) -> T;
}

struct Rectangle_2<T: Mul> {
  x: T,
  y: T,
}

// Where T uses mult triat and output must be of type T and implements Copy
impl <T:Mul<Output = T> + Copy> Shape_2<T> for Rectangle_2<T> {
  fn area(&self) -> T {
    self.x * self.y
  }
}

// using copy trait makes all uses set to var a copy
// with clone must use .clone() syntax
#[derive(Debug, Clone, Copy)]
struct A(i32);

// comparison operator traits
#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct B(f32);

trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

pub fn run() {

}