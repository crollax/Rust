pub use crate::restaurant::front_of_house::hosting;

pub fn eat_at_restaurant() {
  // front_of_house::hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  front_of_house::serving::take_order("smothered and covered");
  back_of_house::fix_incorrect_order();
  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");

  println!("I'd like {} toast please", meal.toast);
}


fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {
      println!("INSIDE of add_to_waitlist()");
    }

    fn _seat_at_table() {}
  }

  pub mod serving {
    pub fn take_order(order:&str) {
      println!("Taking this order: {}", &order);
    }

    fn _serve_order() {}

    fn _take_payment() {}
  }
}

fn serve_order() {
  println!("INSIDE serve_order()");
}

mod back_of_house {
  pub fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
  
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast:&str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

