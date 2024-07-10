struct BackAccount {
    owner: String,
    balance: f64,
}

impl BackAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}.",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Acount owned by {} has a balance of {}.",
            self.owner, self.balance
        );
    }
}

fn main() {
    let mut account = BackAccount {
        owner: "Alice".to_string(),
        balance: 155.5,
    };

    account.check_balance();
    account.withdraw(45.5);
    account.check_balance();
}
