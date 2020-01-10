
pub fn run () {
  for_loops();
}

fn for_loops () {
  let a = vec![10,20,30,40,50];

  for i in a { 
    println!("i: {}", i);
  }

  // Exclusive ranges
    // ..= -> Inclusive ranges (experimental)
  for i in 1..11 {
    println!("i: {}", i);
  }
}

fn while_loops () {
  let mut n = 10;

  while n != 0 {
    println!("{}!", n);
    n = n - 1;
  }
}

fn named_loops () {
  'a: loop {
    println!("loop a");
    'b: loop {
      println!("loop b");
      'c: loop {
        println!("loop c");

        break 'b
      }
    }
    continue
  }
}

fn loop_value () {
  let x = loop {
    break 10;
  };

  println!("x: {}", x);
}