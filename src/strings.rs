//Primitive str= Immutable fixed-length string somewhere in memory
//String= Growable, heap allocated data structure- Use when you need to midify or own string data

pub fn run(){
    let mut hello=String::from("Hello");

    //Get Length
    println!("Length:{}",hello.len());
    
    //Push char
    hello.push('W');

    //Push string
    hello.push_str("orld");

    //Capacity in bytes
    println!("Capacity:{}",hello.capacity());

    //Check if empty
    println!("Is Empty:{}",hello.is_empty());

    //Contains
    println!("Contains 'World' {}",hello.contains("World"));

    //Replace
    println!("Replace:{}",hello.replace("World", "There"));

    //Loop through string by white space
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //Create string with capacity
    let mut s=String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertion testing - Use when to test to check eqauls to for 2 diff things
    assert_eq!(2,s.len());
    assert_eq!(11,s.capacity());

    println!("{}",hello);

}