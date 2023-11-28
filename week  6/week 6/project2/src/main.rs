use std::io;
fn main() {
    let mut  name=String::new();
    io::stdin().read_line(&mut name)
    .expect("invalid name");

    
println!("                                                       WELCOME {name}" );
println!("\n How many siblings do you have");

let mut n=String::new();
io::stdin().read_line(&mut n)
.expect("please enter a valid number");

let sibling_number:i32=n.trim().parse()
.expect("failed to parse the string");

println!("PLEASE ENTER THE FIRST NAME OF SIBLING"); 

    let  sibling_first_name_array:[&str;60]=["0"; 60];

    for i in 0..=60 {
        println!("ENTER THE FIRST NAME  {}:", i + 1);
     

    let mut sibling_name=String::new();
    io::stdin().read_line(&mut name)
    .expect("invalid name");
    }

    println!(" HW OLD IS EACH SIBLING (RESPECTIVE TO INPUT) ");

    let mut sibling_age_array:[i32;60]=[0;60];

    for j in 0..60{

        println!("ENTER AGES OF SIBLINGS  {}:", j  + 1);
    }

    let mut sibling_age=String::new();
    io::stdin()
    .read_line(&mut sibling_age)
    .expect("invalid age");

    let age:i32=sibling_age.trim().parse()
    .expect("not a valid age");

    
    }

 

