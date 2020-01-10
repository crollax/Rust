pub fn run () {
  
  let mut numbers: [i32; 5] = [1,2,3,4,5];

  println!("numbers array = {:?}", numbers);
  println!("numbers occupies {} bytes", std::mem::size_of_val(&numbers));

  numbers[2] = 20;

  // must use & (generic) before type [i32]
    // equals whatever size the array is that is set to it
  // &numbers[0(startINDEX)..5(numOfItems)]
  let slice: &[i32] = &numbers[0..5];
  println!("Slice of numbers = {:?}", slice);

}