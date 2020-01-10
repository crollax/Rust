pub fn run () {
  using_ref();
}

fn using_ref () {
    // using ref
    let u = 10;
    // the next two statements are equivlent
    let v = &u;
    let ref z = u;
  
    if z == v {
      println!("They are equal");
    }
}