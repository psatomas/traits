#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
    
}

impl Copy for Duration {}

fn main() {
    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;

    println!("{:?}", one_hour);
}