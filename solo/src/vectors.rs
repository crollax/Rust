// Vecs
  // Resizable Arrays
  // Made up of three data points
    // 1) Pointer to the data
    // 2) length
    // 3) Capacity
  // Must have the same type
    // Can use Enums to circumvent

use std::cmp::PartialOrd;

pub fn run () {
  let mut numbers: Vec<i32> = vec![1,2,3,4,5];

  numbers.push(6);

  println!("Vec numbers = {:?}", numbers);
  println!("Vec numbers occupies {} bytes", std::mem::size_of_val(&numbers));

  // must use &(generic) before type [i32]
    // equals whatever size the array is that is set to it
  // &numbers[0(startINDEX)..5(numOfItems)]
  let slice: &[i32] = &numbers[1..3];
  println!("Slice of Vec numbers = {:?}", slice);

  for vec in numbers.iter_mut() {
    *vec *= 2;
  }

  println!("Vec numbers after iter_mut() {:?}", numbers);

  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);
  // let largest = largest(&numbers);
  // println!("Largest item in Vec = {}", largest);

}

#[derive(Debug)]
enum Example {
  Float(f64),
  Int(i32),
  Text(String),
}

pub fn run_2 () {
  let mut v: Vec<i32> = Vec::new();

  for i in &v {
    println!("{}", i);
  }

  println!("{:?} {} {}", &v, v.len(), v.capacity());
  println!("{:?}", v.pop());

  let r = vec![
    Example::Int(142),
    Example::Float(12.32),
    Example::Text(String::from("string")),
  ];
  println!("{:?}", &r);
}

pub fn largest <T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list.iter() {
    if &item > &largest {
      largest = &item;
    }
  }

  &largest
}

pub fn make_vecs() {
  println!("-- Making Vecs");
  let mut v = Vec::new();
  v.push("some value");
  v.push("another value");

  let mut v2 = vec![1,2,3];
  v2.push(4);

  println!("v = {:?} and v2 = {:?}", &v, &v2);
}

