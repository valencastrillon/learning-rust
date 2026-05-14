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