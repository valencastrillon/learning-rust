// ============================================================
// HOMEWORK: Safe Data Transfer Program
// Topics: Ownership transfer, Borrowing & References
// ============================================================

fn main() {
    println!("=== Safe Data Transfer Program ===\n");

    // --------------------------------------------------------
    // PART 1: Passing data between functions (Ownership Transfer)
    // --------------------------------------------------------
    println!("--- PART 1: Ownership Transfer ---");

    let message = String::from("Hello from main!");

    // Ownership of `message` is MOVED into the function.
    // After this call, `message` is no longer valid here.
    let processed = process_message(message);

    // println!("{}", message); // ❌ ERROR: value moved, can't use it
    println!("Processed: {}", processed);

    // --------------------------------------------------------
    // PART 2: Observing Ownership Transfer
    // --------------------------------------------------------
    println!("\n--- PART 2: Observing Ownership ---");

    let data = String::from("Sensitive Data");
    println!("Before transfer: {}", data);

    // Transfer ownership to another variable
    let new_owner = data;
    // println!("{}", data);  // ❌ ERROR: `data` was moved to `new_owner`
    println!("New owner has: {}", new_owner);

    // Passing ownership into a function and getting it back
    let name = String::from("Rustacean");
    let name = take_and_return(name); // ownership goes in and comes back
    println!("Got back: {}", name);   // ✅ works because it was returned

    // --------------------------------------------------------
    // PART 3: Fixing Ownership Errors with Borrowing / References
    // --------------------------------------------------------
    println!("\n--- PART 3: Borrowing & References ---");

    let user = String::from("Alice");

    // Pass an IMMUTABLE reference (&) — no ownership transfer
    greet_user(&user);
    println!("Still have user after borrow: {}", user); // ✅ still valid!

    // Multiple immutable borrows are allowed simultaneously
    let score: i32 = 95;
    let a = &score;
    let b = &score;
    println!("Same score accessed twice: {} and {}", a, b);

    // MUTABLE reference — allows the function to modify the value
    let mut balance = 100.0_f64;
    println!("Balance before deposit: ${:.2}", balance);
    deposit(&mut balance, 50.0);
    println!("Balance after deposit:  ${:.2}", balance);

    // RULE: only ONE mutable reference at a time
    // let r1 = &mut balance;
    // let r2 = &mut balance; // ❌ ERROR: can't have two mutable borrows

    // --------------------------------------------------------
    // PART 4: Real-world example — transferring a "file"
    // --------------------------------------------------------
    println!("\n--- PART 4: Simulated File Transfer ---");

    let file = FileData {
        name: String::from("report.txt"),
        content: String::from("Q1 results: all good."),
    };

    // Borrow to read without giving up ownership
    print_file_info(&file);

    // Transfer ownership to the archive (simulate sending the file)
    let archived = archive_file(file);
    println!("Archived: {}", archived);

    // println!("{}", file.name); // ❌ ERROR: `file` was moved
}

// ============================================================
// HELPER FUNCTIONS
// ============================================================

/// Takes ownership of the String, modifies it, and returns it.
fn process_message(mut msg: String) -> String {
    msg.push_str(" [processed]");
    msg // ownership is returned to the caller
}

/// Takes ownership and returns it so the caller gets it back.
fn take_and_return(s: String) -> String {
    println!("  Function received: {}", s);
    s // move back to caller
}

/// Borrows the String immutably — caller keeps ownership.
fn greet_user(name: &String) {
    println!("Hello, {}!", name);
}

/// Borrows the f64 mutably — can change the value in place.
fn deposit(balance: &mut f64, amount: f64) {
    *balance += amount; // dereference to modify the actual value
}

// ============================================================
// STRUCT EXAMPLE
// ============================================================

struct FileData {
    name: String,
    content: String,
}

/// Borrows the file — caller still owns it afterward.
fn print_file_info(file: &FileData) {
    println!("File: {} | Size: {} bytes", file.name, file.content.len());
}

/// Takes ownership of the file (simulates archiving / sending).
fn archive_file(file: FileData) -> String {
    format!("[ARCHIVED] {} — content: \"{}\"", file.name, file.content)
}



/* Mini Project
fn main() {
    let name: String = String::from("Valen");
    greeting(&name); 
    shout(&name); 
}

fn greeting(name: &String) {
    println!("Hello, {}!", name);
}

fn shout(name: &String) {
    println!("HEY, {}!", name.to_uppercase());
}
*/



/* Borrowing and lifetimes
fn main() {
    let x :i32 = 6;
    let mut s :String = String::from(s: "hello");
}

fn dangle() -> &String {
    let s :String = String::from(s: "hello");
    &s // This returns a reference to s, but s will be dropped when the function ends
}

fn no_dangle() -> String {
    let s :String = String::from(s: "hello");
    s // Ownership is moved out - safe!
}
*/



/* Mutable References
fn main() {
    let s :String = String::from(s: "hello"); 
    let len :usize = calculate_length(&s); // Pass a reference with &
    //Shared References - &

    let mut ss :String = String::from(s: "hello");
    add_exclamation(&mut ss);
}

fn calculate_length(s: &String) -> usize {
    s.len() // We can read s, but we don't own it
}

fn add_exclamation(s: &String) {
    (&mut*s).push_str(string: "!"); // We can modify through a mutable reference
}
*/



/* Ownership and References
fn main() {
    //clone
    let s :String = String::from(s: "hello");
    let s2 :&String = &s; // world
    println!("{}", s); 



}

//A reference lets you borrow data without taking ownership of it.
//&
//&mut
*/



/* 
fn main() {
    let s :String = String::from(s: "hello"); 
    // assigned to a vari
    // fx

    takes_ownership(s); 

    println!("{}", s); 
    let x :i32 = 5;
    makes_copy(x);

    println!("{}", x);

}

fn takes_ownership(s: String) {
    println!("{}", s);
} //s is dropped here - memory freed

fn makes_copy(x: i32) {
    println!("{}", x);
} //x g
*/



/* Ownership and Move Semantics
fn  main () { 
    let x :i32 = 5; 
    let y :i32 = x; // x is COPIED into y

    //Both x and y are valid! x was not moved.
    println!("x: {}, y: {}", x, y);    
    
    let s1 :String = String :: from (s: "hello" ); 
    let s2 :String = s1; // s1 is MOVED into s2

    //s1 is no longer valid after this point!
    println!("{}", s1); // This works fine, but we can't use s1 anymore
    
    //passed to a fn
    //assigne var

}

fn number(i: i32) -> i32 {
    i + 1
}
*/



/* Bucles (loop)
fn main() {
    let mut count: i32 = 1;
    loop{
    println!("{}", count);
    if count == 5 {
    break;
    }
    count+=1;
}
}
*/



/* Bucles (while) 
fn main(){
    let mut count: i32 = 1;
    while count<=5 {
        println!("{}",count);
        count+=1;
}
*/



/* Bucles (para que vaya hasta el 5, colocarle un =5)
fn main(){
    for i in 1..5 {
        println!("Round {}", i);
    }
}
*/



/* Condicionales
fn main(){
    let age: i32 = 18;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }
}
*/



/* Funciones
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main(){
    let result: i32 = add(2, 3);
    println!("{}", result);
}
*/



/* Saludos 
fn greet(name: &str) {
    println!("Heyyy {}", name)
}

fn main() {
    greet("Valen");
}
*/