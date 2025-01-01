fn main() {
    println!("Hello, world!");
}



//variables and mutuability
fn variables(){
    /*
    // wont work because variables are on default not mutuable
    let x = 20;
    x = 20; 
    */

    //assign mutuable keyword to variable to make it changable
    let mut x = 20;
    let x = 20;
    

    
    //similar to unmutuable variables are constant variables, the difference is that constants never change, constant require a type
    const y: u32 = 20;


    //you can overshadow variables which deletes the last value completely and lets you assign an entirly new value to your choosing
    let z = 30;
    let z = "twenty";
}

fn dataTypes(){
    // ========== Scalar Types ==========

    // Integers (signed and unsigned)
    let signed_integer: i32 = -42; // A signed 32-bit integer
    let unsigned_integer: u32 = 42; // An unsigned 32-bit integer (no negatives)

    // Floating-point numbers
    let float_number: f64 = 3.1415; // A 64-bit floating-point number
    let small_float: f32 = 2.718;  // A 32-bit floating-point number

    // Booleans
    let is_true: bool = true;  // Boolean type, true or false
    let is_false: bool = false;

    // Characters
    let letter: char = 'A';    // A single Unicode scalar value
    let emoji: char = '😊';    // Supports emojis and other Unicode characters

    // ========== Compound Types ==========

    // Tuples (fixed-size, heterogeneous collection)
    let tuple_example: (i32, f64, char) = (42, 6.28, 'R');
    let (x, y, z) = tuple_example; // Destructure the tuple

    // Arrays (fixed-size, homogeneous collection)
    let array_example: [i32; 4] = [1, 2, 3, 4]; // Array of 4 integers
    let repeated_array = [0; 5]; // Array with 5 elements, all set to 0

    // Slices (dynamically sized view into an array)
    let slice: &[i32] = &array_example[1..3]; // Slice of the array

    // ========== Text Types ==========

    // String slices (&str)
    let string_slice: &str = "Hello, world!"; // Immutable reference to a string

    // String (heap-allocated, growable string)
    let mut string_example = String::from("Hello"); // Mutable string
    string_example.push_str(", Rust!");

    // ========== Collections (not primitive types) ==========

    // Vector (growable array)
    let mut vector_example: Vec<i32> = vec![1, 2, 3];
    vector_example.push(4); // Add an element to the vector

    // ========== Special Types ==========

    // Option (used for values that might be absent)
    let some_value: Option<i32> = Some(42); // Some contains a value
    let no_value: Option<i32> = None;       // None represents absence of value

    // Result (used for error handling)
    let result: Result<i32, &str> = Ok(42); // Ok represents success
    let error: Result<i32, &str> = Err("Something went wrong"); // Err represents failure

    // ========== Miscellaneous ==========

    // Unit type ()
    let unit: () = (); // Represents an empty value or nothing

    // Never type (!)
    // fn never_returns() -> ! { panic!("This function never returns!"); }

    // Raw pointer (unsafe code, rarely used)
    let raw_pointer: *const i32 = &signed_integer;

    // Printing examples
    println!("Signed integer: {}", signed_integer);
    println!("Float: {}", float_number);
    println!("Boolean: {}", is_true);
    println!("Character: {}", emoji);
    println!("Tuple: {:?}", tuple_example);
    println!("Array: {:?}", array_example);
    println!("String: {}", string_example);
    println!("Vector: {:?}", vector_example);
    println!("Option Some: {:?}", some_value);
    println!("Option None: {:?}", no_value);
    println!("Result Ok: {:?}", result);
    println!("Result Err: {:?}", error);
}
