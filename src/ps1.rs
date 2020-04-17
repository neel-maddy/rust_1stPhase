
use std::vec::Vec;
pub fn run() {

    let mut numbers:Vec<i32>=vec![10,7,100,87,99,65,2,1,34,0];
    numbers.sort();
    println!("Assending order sort: {:?}", numbers);
    numbers.reverse();
    println!("Desending order sort: {:?}", numbers);
}