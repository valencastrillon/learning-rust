use std::collections::HashMap;

let mut account = HashMap::new();

account.insert("alice".to_string(), 12);
account.entry("alice".to_string()).or_insert(20);

HashMap<K,V>
Option<T>
Vec<T>









//vectors

let v: Vec<i32> = Vec::new();

let mut v = Vec::new(); //in the future we want to add values

v.push(5);

[] ...

let v = vec![2, 3, 5]; //2, 3, 5

{
    let v = vec![2, 3, 5];
} -->.
v .. is invalid

let third: &i32 = &v[2]
let value_not = &v[100]; 

let get_value = v.get(1);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

String &str

let x = String : : from("&!");
let c = "you".to_string();

let mut y = String::from("hello");
y.push_str(" there");
//hello there
println!
let hello = "how are you"; //&str

let y = &hello[0..4]


for leter in hello.chars() {
    println!("{}", letter);
}

Result<T, E>
