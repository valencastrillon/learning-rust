struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    //let mut user1 = build_user(String::from("alice@example.com"), String::from("alice"));

};    

   // println!("User: {}, Email: {}, Sign-in Count: {}", user1.username, user1.email, user1.sign_in_count);
}

fn main() {
    email: new_email,
    ..user1
}

let color(i32, i32, i32);
let point(i32, i32, i32); //ejemplo

struct User {
    username: &str,
    email: String,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main () {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area of the rectangle {}", rect.area()),
}

fn area (){
    width * height;
}

#[derive(Debug)]          // Add this attribute
struct Rectangle {
    width: u32,
    height: u32,
}   

println!("{rect1:?}";
println!("rect1 is {:#?}"); 

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect1 = Rectangle {width: 30, height: 50,};
println!("Area: {}", rect1.area());               // Method syntax
 
//-----------------------------------------------------------------------------

struct Wallet {
    balance : u64;  
}

impl Wallet {
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }
}

fn main() {
    let mut my_wallet = Wallet { balance: 400 };
    my_wallet.deposit(1000);
    println!("Balance: {}", my_wallet.balance);
}