/**
 * Variables hold primitve data or references to data
 * Variables are immutable by default
 * Rust is a block-scoped language
 */

pub fn run(){
    let name = "Amado";
    let mut age = 23;
    println!("My name is {} and I am {}", name, age);
    age = 24;
    println!("My name is {} and I am {}", name, age);

    //Defne constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Amado", 23);
    println!("{} is {}", my_name, my_age);
}