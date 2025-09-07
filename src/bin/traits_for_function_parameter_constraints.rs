use std::collections::HashMap;

trait Acommodation {
    fn get_description(&self) -> String{
        String::from("A wonderful place to stay")
    }
    fn book(&mut self, name: &str, nights: u32);
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

impl Acommodation for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

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

impl Acommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

fn book_for_one_night(entity: &mut impl Acommodation, guest: &str) {
    entity.book(guest, 1);;
}
fn main() {
    let mut hotel = Hotel::new("The Luxe");
    book_for_one_night(&mut hotel, "Piers");
    println!("{hotel:#?}");
    
    let mut airbnb = AirBnB::new("Peter");
    book_for_one_night(&mut airbnb, "amanda");
    println!("{airbnb:#?}");
}