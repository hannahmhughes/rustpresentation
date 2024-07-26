// This is the main function.
fn main() {
    // Print text to the console.
    println!("Hello World!"); // Hello World!

    // Let's try adding two numbers
    let a = 1; 
    let b = 2; 
    println!("{}", a + b); // 3

    // Notice that println! is a macro! 
    // In the background, println looks like this: 
    /* 
    macro_rules! println {
        () => { ... };
        ($($arg:tt)*) => { ... };
    }
    */

    // Create an immutable array 
    let array = ["a", "b", "c"]; 
    println!("{:?}", array);  // ["a", "b", "c"]

    // Create a mutable vector 
    let mut vec = vec!["d", "e"];
    println!("{:?}", vec);  // ["d", "e"]

    // Add an element to the vector 
    vec.push("f");
    println!("{:?}", vec); // ["d", "e", "f"]

    for x in &vec {
        println!("{x}"); // d
                         // e
                         // f
    }

    // Compare the vector and the array
    if vec[1] == array[1] {
        println!("{:?} and {:?} are equal", vec[1], array[1]); 
    }  else {
        println!("{:?} and {:?} are not equal", vec[1], array[1]); 
        // "e" and "b" are not equal
    }
}

