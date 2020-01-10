
// lifetimes are specified with '
fn pr<'a> (x: &'a str, y: &'a str) -> &'a str {
  if x.len() == y.len() {
    x 
  } else {
    y    
  }
}

pub fn run() {
  let a = "a string";
  let b = "b string";

  // Static lifetimes live for program entirety
    // Should avoid in most cases. Slows program
  let s: &'static str = "The long lasting string";

  let c = pr(a, b);

  println!("{}", c);
}