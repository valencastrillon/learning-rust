/* Panic Example
fn withdraw(balance: i32, amount: i32) -> i32 {
    if amount > balance {
        panic!("Insufficient funds");
}
    balance - amount
}

fn main() {
    let balance = 100;
    let amount = 200;
    let new_balance = withdraw(balance, amount);
    println!("New balance: {}", new_balance);
}
*/



/* Result <T, E> Example
fn withdraw(balance: i32, amount: i32) -> Result<i32, String> {
    if amount > balance {
        Err("Insufficient funds".to_string());
    } else{
    Ok(balance - amount)
    }
}

fn main () {
    let balance = 100;
    let amount = 200;
    let result: Result<i32, String> =  withdraw(balance, amount);
    
    match result {
        Ok(new_balance: i32) => {
            println!("Withdrawl succesful");
            println!("New balance: {}", new_balance);
        }
        Err(error: String) => {
            println!("Withdrawal failes");
        }
    }
}
*/

/*?Without 
fn withdraw(balance: i32, amount: i32) -> Result<i32, String> {
    if amount > balance {
        Err("Insufficient funds".to_string())
    } else {
        Ok(balance - amount)
    }
}

fn process_transaction() -> Result<i32, String> {
    let result: Result<i32, String> = withdraw(balance: 100, amount: 50);
    
    match result {
        Ok(new_balance: i32) => Ok(new_balance),
        Err(error: String) => Err(error),
    }
}

fn main() {
    let result: Result<i32, String> = process_transaction();
    match result {
        Ok(new_balance: i32) => {
            println!("Transaction successful");
            println!("Remaining balance: {}", new_balance);
        }
        Err(error: String) => {
            println!("Transaction failed");
            println!("Reason: {}", error);
        }    
    }
}
*/

/* Custom Error with enums
#[derive(Debug)]
enum BankError {
    InsufficientFunds,
    InvalidTransaction,
    UnknownAccount,
}

fn withdraw(balance: i32, amount: i32) -> Result<i32, BankError> {
    if amount <= 0 {
        Err(BankError::InvalidTransaction)
    } else if amount > balance {
        Err(BankError::InsufficientFunds)
    } else {
        Ok(balance - amount)
    }
}

fn find_account(account_exists: bool) -> Result<String, BankError> {
    if account_exists {
        Ok("Account found".to_string())
    } else {
        Err(BankError::UnknownAccount)
    }
}

fn main() {
    let result: Result<i32, BankError> = withdraw(balance: 100, amount: 150);
    match result {
        Ok(new_balance: i32) => {
            println!("Withdrawal successful");
            println!("Remaining balance: {}", new_balance);
        }
        Err(error: BankError) => {

            match error {
                BankError::InsufficientFunds => {
                println!("Error: Insufficient funds");
                }

                BankError::InvalidTransaction => {
                println!("Error: Invalid transaction");
                }

                BankError::UnknownAccount => {
                    println!("Error: Unknown account");
                }
            } 
        }
    }

    let account_result: Result<String, BankError> = find_account(account_exists: false);
    match account_result {
        Ok(message: String) => {
            println!("{}", message);
        }
        Err(error: BankError) => {
            match error {
                BankError::InsufficientFunds => {
                    println!("Error: Insufficient funds");
                }
                BankError::InvalidTransaction => {
                    println!("Error: Invalid transaction");
                }
                BankError::UnknownAccount => {
                    println!("Error: Unknown account");
                }
            }
        }
    }
}
*/


