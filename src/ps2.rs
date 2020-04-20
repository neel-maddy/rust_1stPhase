pub fn run(){
    let mut numbers:Vec<i32>=vec![10,7,100,87,99,65,2,1,34,0];
    numbers.sort();
    println!("Assending order sort: {:?}", numbers);

    //Smallest Number
    println!("Smallest Number: {}", numbers[0]);

    numbers.reverse();
    println!("Desending order sort: {:?}", numbers);

    //Largest Number
    println!("Largest Number: {}", numbers[0]);


    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }
}