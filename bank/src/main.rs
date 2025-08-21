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
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
}

impl Bank{
    fn new() -> Self{
        Bank{accounts: vec![]}
    }
}

fn print_account(account: &Account){
    println!("current account {:#?}", account);
}

fn add_account(bank: &mut Bank, account: Account){
    bank.accounts.push(account);
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(
        1, String::from("me")
    );

    println!("current bank status {:#?}", bank);

    // pass by reference
    print_account(&account);
    print_account(&account);

    add_account(&mut bank, account);
    println!("current bank status {:#?}", bank);
}
