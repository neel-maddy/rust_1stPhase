//Vectors- Resizable Arrays
use std::mem;

pub fn run() {
    let mut numbers:Vec<i32>=vec![1,2,3,4];
    println!("{:?}", numbers);

    //Re-assign values
    numbers[2]=20;
    println!("{:?}",numbers);

    //Add on to vectors
    numbers.push(5);
    numbers.push(6);

    //Pop off the last value
    numbers.pop();

    
    //Get single value
    println!("Single Value: {}",numbers[2]);

    //Get Vector length
    println!("Vector Length: {}",numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes.", mem::size_of_val(&numbers));

    //Get Slice
    let slice:&[i32]=&numbers;
    println!("Slice: {:?}",slice);

    let slice:&[i32]=&numbers[1..3];
    println!("Slice: {:?}",slice);

    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //Loop & mutate values
    //For Java Script it's like mapping 
    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("Numbers Vec:{:?}",numbers);
    

}