use rand::Rng;
mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
pub mod hosting {
    pub fn add_to_waitlist() {}
}

fn main(){

  println!("hello world");
  let secret_number = rand::thread_rng().gen_range(1..=100);

}
