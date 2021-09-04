pub fn run() {
    // Print to Console
    println!("Hello from print.rs");
    //Basic Formating
    println!("Number: {}{}{}", 1, 2 ,3);
    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");
    //Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");
    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //Placeholder Debug Trait
    println!("{:?}", [12, 13, 14]);
}