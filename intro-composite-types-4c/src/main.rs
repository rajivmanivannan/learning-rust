fn main() {
//Tuples.
//Return a set of different types of vlues.
let a = (5,"Hello, world",false);
println!("The first value are: {} {}",a.0,a.1);

//How do you access multiple values at once?
let a= (1,2,3);

//Option 1
let val_one = a.0;
let val_two = a.1;
let _val_three = a.2;

println!("The first value is: {} The second value is {}",val_one,val_two);

//Option 2 Destructuring

let (val_1,val_2,_val_3) = a;
println!("The first value is: {} The second value is {}",val_1,val_2);

//Arrays
//A set of same type of values.

let ar= [0;10];
println!("The array is: {:?}",ar);

//Mutablity
let mut am = [0];
println!("The value is: {}",am[0]);

am[0] =1;
println!("The updated value is: {}",am[0]);




}
