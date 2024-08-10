// Encapsulation, visibility, public/private fields
fn main() {
    let mut my_bank = BankAccount {
        bal: 0.0,
        acc_num: 69,
    };
    my_bank.see_balance();
    println!("{}", my_bank.deposit(225.57));
    my_bank.see_balance();
    println!("{}", my_bank.withdraw(36.4));
    my_bank.see_balance();
}

struct BankAccount {
    bal: f64,
    acc_num: i16,
}
impl BankAccount {
    fn deposit(&mut self, dep: f64) -> f64 {
        self.bal += dep;
        self.bal
    }
    fn withdraw(&mut self, wd: f64) -> f64 {
        self.bal -= wd;
        self.bal
    }
    fn see_balance(&self) -> () {
        println!("Balance for account number {}: £{}", self.acc_num, self.bal)
    }
}
