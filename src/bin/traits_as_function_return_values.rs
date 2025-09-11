use std::collections::HashMap;

trait Accommodation {    
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String{
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl Description for Hotel {}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str,)
where 
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 1);
}

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Luxe ")
}
fn main() {
    let mut hotel = choose_best_place_to_stay();    
    let mut airbnb = AirBnB::new("Peter");
    mix_and_match(&mut hotel, &mut airbnb, "Piers");
}