#[derive(Debug)]
struct Account{
    id : u32,
    holder : String,
    balance: i32
}

impl Account{
    fn new(id: u32, holder: String) -> Self{
        Account{
            id, 
            holder,
            balance: 0
        }
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }
    
    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    } 

    fn withdraw(&mut self, amount: i32) -> i32 { 
        self.balance -= amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
}

impl Bank{
    fn new() -> Self{
        Bank{accounts: vec![]}
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> { 
        self.accounts.iter().map(|account| account.summary()).collect::<Vec<String>>()
    }

    fn add_account(&mut self, account: Account){
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Moos"));
    let mut account2 = Account::new(2, String::from("John"));

    account.deposit(500);
    account.withdraw(250);
    account2.deposit(900);

    bank.add_account(account);
    bank.add_account(account2);

    println!("current bank summary: {:#?}", bank.summary());
    println!("current bank balance: {}", bank.total_balance());
}
