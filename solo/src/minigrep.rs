use std::io;

pub fn run() {

  let mut args: Vec<String> = Vec::new();
  let mut age = String::new();


  println!("What is your age?");

  io::stdin().read_line(&mut age)
    .expect("Failed to read line");

  args.push(age);

  println!("{:?}", &args[0]);


}