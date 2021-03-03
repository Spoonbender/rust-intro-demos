enum AccountStatus {
    Active,
    Terminated,
}

struct Account {
    id: u64,
    status: AccountStatus,
    balance: u64,
}

fn deposit(account: &mut Account, amount: u64) {
    match account.status {
        AccountStatus::Active => account.balance += amount,
        AccountStatus::Terminated => {
            println!("Can't deposit to terminated account")
        }
    }
}

fn withdraw(account: &mut Account, amount: u64) {
    match account.status {
        AccountStatus::Active => account.balance -= amount,
        AccountStatus::Terminated => {
            println!("Can't withdraw from terminated account")
        }
    }
}

fn main() {
    let mut account = Account {
        id: 1,
        status: AccountStatus::Active,
        balance: 0,
    };

    deposit(&mut account, 100);
    withdraw(&mut account, 70);
    println!("ID: {}, Balance: {}", account.id, account.balance);

    account.status = AccountStatus::Terminated;
}
