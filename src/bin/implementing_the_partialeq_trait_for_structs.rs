#[derive(PartialEq, Debug)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
    
}

// impl PartialEq for Flight {
//     fn eq(&self, other: &Self) -> bool {
//         self.origin == other.origin
//             && self.destination == other.destination
//     }
    
// }

fn main () {
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:20");
    let c = Flight::new("New York", "Los Angeles", "8:00");
    let d = Flight::new("New York", "London", "08:00");
    println!("{}", a == b);
    println!("{}", a == d);
    println!("{}", a.eq(&b));
    println!("{}", a.ne(&b));
    println!("{}", a == c);
    println!("{}", a != c);
}