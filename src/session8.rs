struct Cat {
    pub name: String,
}

struct Dog {
    pub name: String,
}

impl Cat{
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl Dog{
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

pub trait Animal {
    fn speak() -> ();
}

impl Animal for Cat {
    fn speak() -> () {
        println!("Meow!");
    }
}

impl Animal for Dog {
    fn speak() -> () {
        println!("Woof!");
    }
}

//we already know that the animal should have &Animal.
fn speak(animal: Box<dyn Animal) {
    //we can just access the trait fns here.
    animal.speak();
}

fn main() {
    //pointers
    //Box pointer in the animals

    let cat_in_box = Box::new(Cat::new("ami"));
    //stack | heap
    //&str | String
    //
    //String.push_slice(&str)
    //
    //cat_pointer is stored in the stack,
    //cat struct and the funcs are stored in the heap.

    speak(cat_in_box);
}