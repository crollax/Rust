#[derive(Debug)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}



impl Message {
    fn call(&self) {
        // Need to evaluate &self::Write()
        println!("{:?}", &self);
    }
}
    
pub fn message() {
    let  m = Message::Write(String::from("hello new message"));
    m.call();
}
