pub fn coin() {
    let c = Coin::value_in_cents(Coin::Quarter(UsState::Alaska));
    c
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Georgia,
    Kentucky,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}
