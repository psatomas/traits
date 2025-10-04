use std::ops::Add;

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

impl Add for Lunch {
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
    
}

fn main() { 
    let one = Lunch { cost: 19.99 };
    let two = Lunch { cost: 29.99 };
    println!("{:?}", one + two);
}