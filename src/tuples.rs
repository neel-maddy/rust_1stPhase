//Tuples group together values of different types
//MAx 12 elements

pub fn run() {
    let person:(&str,&str,i8)=("Maddy","Keonjhar",28);

    println!("{} is from {} and is {}.",person.0,person.1,person.2);
    
}