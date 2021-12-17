fn main() {
    //Match statement
    // Mix of switch and if.
    let x =1;

    match x {
        1 => println!("Value of x is 1"),
        2 => println!("Value of x is 2"),
        3 => println!("Value of x is 3"),
        _ => println!("Value of x is invalid"),
    }

    // Exhaustive matches

    let y = true;
    let z = true;

    //Possible values:
    // true,true
    // false,false
    // false,true
    // true,false

    match (y,z){
        (true,true) => println!("y and z are true"),
        (false,false) => println!("y and z are false"),
        (false,true) => println!("y is false and z is true"),
        (true,false) => println!("y is true and z is false"),
    }


}
