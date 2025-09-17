trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64;
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
    
}

#[derive(Debug)]
struct Bonus {
    amount: f64
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;

    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
    
}




fn main() {
    let income = Income { amount: 50000.50 };
    println!("Total tax owed: ${:.2}", income.tax_bill());

    let bonus = Bonus { amount: 10000.23 };
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
}