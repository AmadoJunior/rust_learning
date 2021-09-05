// Vectors are resizeable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    numbers[0] = 0;
    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);
    //Get at index
    println!("{}", numbers[0]);
    //Get vector length 
    println!("Vec length: {}", numbers.len());
    //Vecs are stack allocated
    println!("Vec occupies {} bytes", mem::size_of_val(&numbers));
    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }
    //Loop and mutate
    for x in numbers.iter_mut(){
        *x *= 2;
        println!("Mutated Num: {}", x);
    }
}