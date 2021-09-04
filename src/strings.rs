/**
 * primitive str = Immutable fixed-length string somewhere in memory
 * String = Growable, heap-allocated data structure - Use when yo need to modify or own string data.
 */

pub fn run(){
    //primitive str
    let hello = "Hello";
    //String
    let mut hello_world = String::from("Hello World!");
    // Get length works on both
    println!("Length: {}, {}", hello.len(), hello_world.len());

    //Push Char/ Strings
    hello_world.push('W');
    hello_world.push_str("orld");

    //Loop through string
    for word in hello_world.split('o') {
        println!("{}", word);
    }

    //Assertion Testing
    assert_eq!(1+2, 4);

    println!("{:?}", hello_world)
}