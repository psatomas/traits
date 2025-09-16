use traits::lodging::{AirBnB, Hotel, Accommodation, Description};
use traits::utils;

fn main() {
    let mut hotel = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel.summarize());
    hotel.book("Dana", 3);

    let mut airbnb = AirBnB::new("Parker");
    println!("{}", airbnb.get_description());
    utils::book_for_one_night(&mut airbnb, "Dan");

    utils::mix_and_match(&mut hotel, &mut airbnb, "Phil");
}
