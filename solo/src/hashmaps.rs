// HashMaps
  // Need consistant types


use std::collections::HashMap;

// enum Score {
//   Color,
//   Value
// }

pub fn run () {

  let mut scores = HashMap::new();
  let mut hm = HashMap::new();

  // let mut score = HashMap::new();


  hm.insert(String::from("random"), 12);
  hm.insert(String::from("strings"), 49);

  for (k, v) in &hm {
    println!("{}: {}", k, v);
  }

  // Use get method with key
    // and destruct to gain access to value
  let key = &String::from("random");
  match hm.get(key) {
    Some(&n) => println!("value: {}", n),
    _ => println!("no match"),
  }

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  println!("HashMap scores {:?}", scores)

}