trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn amount(&self) -> f64;

    fn set_amount(&mut self, new_amount: f64);

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }   
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
       self.amount = new_amount; 
    }
}

#[derive(Debug)]
struct Bonus {
    value: f64
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
    
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
         self.value = new_amount; 
    }
}




fn main() {
    let mut income = Income { amount: 50000.50 };
    println!("Total tax owed: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: ${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 10000.23 };
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
}