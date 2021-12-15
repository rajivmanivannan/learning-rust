fn main() {

// Rust is a statically typed language, which means that it must know the types of all variables at compile time. 

   let x : i32 =6; // What is the type here?
   // A signed 32 bit integer.

   println!("The value of x is {}",x);

    // Type Inference.
    let a  =6;

  // x = "x-men"; // It's can't assign string. Becoz - Static types
    
//Types
   //Scalar Types
   //A scalar type represents a single value. 
   //Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    //Length	Signed	Unsigned
    //8-bit	    i8	    u8
    //16-bit	i16	    u16
    //32-bit	i32	    u32
    //64-bit	i64	    u64
    //128-bit	i128	u128
    //arch	    isize	usize


   //f32 32-bit float
    let y: f32 = 3.0; // f32
    
   //f64 64-bit float
    let x = 2.0; // f64

   //bool boolean
    let t = true;

    let f: bool = false; // with explicit type annotation

   //Character UTF-8 and ASCI 
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';



   // Numeric Operation
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

}
