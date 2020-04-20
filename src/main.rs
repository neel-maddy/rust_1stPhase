use std::mem;

const MEANING_OF_LIFE:u8=42;//No fixed address
static mut z:i32=123;

//mod types;
//mod strings;
//mod tuples;
//mod arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod functions;
//mod pointers_ref;
//mod structs;
//mod enums;
//Test 123
//mod cli;
//mod ps1;
//mod ps2;
//mod operators;
//mod loop10;
//mod scopes;

// fn scope_and_shadowing(){
//     let a=123;
//     {
//         let b=456;
//         println!("inside b = {}",b);

//         let a =777;
//         //A can print inside
//         println!("Inside a={}",a);
//     }
//     println!("a={}",a);
//     //println!("outside of scope b {}",b);
// }


fn main() {
    //println!("Hello, world!");
    //types::run();
    //strings::run();
    //tuples::run();
    //arrays::run();
    //vectors::run();
    //conditionals::run();
    //loops::run();
    //functions::run();
    //pointers_ref::run();
    //structs::run();
    //enums::run();
    //cli::run();
    //ps1::run();
    //ps2::run();
    //operators::run();
    //loop10::run();
    //scopes::run();
    //scope_and_shadowing();
    println!("{}",MEANING_OF_LIFE);
    unsafe{
        println!("Z is {}",z);
        z=888;
        println!("Z is {}",z);
    }
    
}
