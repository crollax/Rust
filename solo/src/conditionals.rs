use std::io;

pub fn run () {

  loop {
    // Get string from input
    let mut age = String::new();
    let check_id: bool = true;
    let mut error: bool = false;

    println!("What is your age?");

    io::stdin().read_line(&mut age)
      .expect("Failed to read line");

    let age: u8 = match age.trim().parse() {
      Ok(num) => num,
      Err(_) => { error = true; 0 },
    };

    if error {
      println!("There was an error");
      continue;
    } 
    else {
      println!("The age you entered is {}", age);
      break;
    }
  }

  
}