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