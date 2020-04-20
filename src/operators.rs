use std::mem;

pub fn run(){
    let mut a =2+3*4;
    println!("a={}",a);
    
    //+= -= *= /=    
    a+=1;
    a-=2;
    println!("a={}",a);
    println!("Remainder of {}/{}={}",a,3,(a%3));

    //Cube
    let a_cubed=i32::pow(a,3);
    println!("{} cubed is {}",a,a_cubed);

    let b=2.5;
    let b_cubed = f64::powi(b,3);
    println!("{} cubed = {}",b,b_cubed);
    let b_to_pi=f64::powf(b, std::f64::consts::PI);
    println!("{}^pi ={}", b,b_to_pi);
    println!("{} cubed = {}, {}^pi ={}",b,b_cubed, b,b_to_pi);

    //bitwise operator only for integers
    // |OR &AND ^XOR !NOR
    //01 OR 10 = 11 == 3_10
    let c=1|2;
    println!("1|2 = {}",c);

    //Shift 
    let two_to_10 = 1 << 10;
    println!("2^10={}",two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //True
    // > <= >= ==
    let x=5;
    let x_is_5 = x==5; //True
    println!("pi_less_4 is {:?}, x is {}, x_is_5 is {:?}",pi_less_4, x,x_is_5);

    
}