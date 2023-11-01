
   use std::io;
fn main() {
    println!("\nStudent Information management System");
    let mut name= String::new();
    io::stdin()
    .read_line(&mut name).expect("failed to read input");
    println!("Your name is {}",name );

     println!("\nEnter your age");
     let mut age = String::new();
     io::stdin().read_line(&mut age).expect("failed to read input");
     let age:i32=age.trim().parse().expect("input not an integer");
     println!("Your age is {}",age );



 
     




}
