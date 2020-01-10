use std::vec;

pub fn run () {
  // let x: u8 = 1;

  // let mut s = String::from("A String");
  // let x = 1;
  // s.pop();

  // println!("{}", s);

  ///////////////////////////////////////////////////
  
  // Compiler will automatically make copies
    /* Rust Stack | Copy types
          bool, character, numbers, slices
          fix sized arrays, touples containing primitives
          function pointers
    */
  let x = 10;
  let y = x;
  let z = x;
  // println!("x = {}", x);
  // println!("y = {}", y);
  // println!("z = {}", z);

  // copy(true);
  // copy("a");
  // copy("a slice");
  // copy(x);
  // copy(String::from("Test")); // causes error
  ///////////////////////////////////////////////////
  
  let mut a = String::from("A String");

  let x = &mut a;
  let y = &mut a;

}

fn copy<T>(t: T) -> T
where // Guard
  T: Copy, // T must ba able to implement Trait Copy
{
  let x = t;
  x
}

fn print(a: u8) {
  println!("value {}", a);
}

fn re(v: Vec<i32>) -> Vec<i32> {
  println!("{}", v[120] + v[111]);
  v
}

fn borrow1(v: &Vec<i32>) {
  // *v == pointer to the ref of the vector arg
  println!("{}", (*v)[10] + (*v)[12]);
}

fn borrow2(v: &Vec<i32>) {
  println!("{}", v[10] + v[11]);
}

fn count(v: &Vec<i32>, val: i32) -> usize {
  v.into_iter().filter(|&&x| x == val).count()
}

pub fn run_2 () {
  let mut v = Vec::new();

  for i in 1..1000 {
    v.push(i);
  }

  // Transfer ownership to re function and then back
  v = re(v);

  println!("Still own v: {} {}", v[0], v[1]);
  
  borrow1(&v);
  println!("Still own v: {} {}", v[0], v[1]);

  borrow2(&v);
  println!("Still own v: {} {}", v[0], v[1]);

  let mut vs = vec![4,5,3,6,7,4,8,6,4,2,4,2,5,3,7,7];
  // Need to sort before dedup
  vs.sort();
  println!("vs sorted: {:?}", &vs);
  // Clone to new set var
  let mut set = vs.clone();
  // Create the set with dedup
  set.dedup();
  println!("set dedup: {:?}", &set);
  println!("vs: {:?}", vs);

  // &i destructures the &{integer} type for i 
  for &i in &set {
    let r = count(&vs, i);
    println!("{} is repeated {} times", i, r);
  }
}