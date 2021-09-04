// Vectors are resizeable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    numbers[0] = 0;
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
}