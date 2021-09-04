// Arrays - Fixed list where elements are the saem data type

use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    numbers[0] = 0;
    println!("{:?}", numbers);
    //Get at index
    println!("{}", numbers[0]);
    //Get array length 
    println!("Array length: {}", numbers.len());
    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}