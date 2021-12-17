fn main() {
   // Loop
   // Multiply x till it is less than 5000

   let mut x = 1;
    loop{
        x = x * 2 ;
        if x > 5000{
            break;
        }
        println!("The value of x now is {}",x);
    }

    //While
    // Multiply x till it is less than 5000
    // Execute some code WHILE the condition is true.
    let mut y = 1;
    while y < 5000{
       y = y * 2 ;
        println!("The value of y now is {}",y);  
    }

    // For Loops
    // They are a little different from other languages.

    for i in 0..10 { //From 0 to 9
        println!("The values of i is {}",i);
    }

    // What if you wanted to write something which is more exlicity?
    for j in 0..=9 { //From 0 to 9
        println!("The values of i is {}",j);
    }

   // For loops
   // Used with iterators

   let z = [1,2,3,4,5];

   for val in z {
    println!("The values of val is {}",val);
   }
}
