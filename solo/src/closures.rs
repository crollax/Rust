use std::thread;
use std::time::Duration;

pub fn run () {
  let intensity = 10;
  let random_number = 5;
  generate_workout(intensity, random_number)
}

pub fn test () {
  let f = |i| i + 1;
  let x = 10;
  println!("{}", f(x));

  let p = || println!("This is a closure");
  p();

  let k = create();
  k();

  // Closure inside of an iterator
  let v = vec![1,2,3];
  println!("is 2 in v? {}", v.iter().any(|&x| x != 2));
}

// Using  move here is like using Copy trait
  // as opposed to using a reference (&)
fn create () -> Box<Fn()> {
  Box::new(move || println!("this is a closure in a box!"))
}


pub fn generate_workout(intensity: u32, random_number: u32) {

  let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  if intensity < 25 {
    println!("Today do {} pushups!", expensive_closure(intensity));
    println!("Next, do {} situps!", expensive_closure(intensity));
  } 
  else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } 
    else {
      println!("Today, run for {} minutes!", expensive_closure(intensity));
    }
  }

}