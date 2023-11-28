use std::io;

fn checker() {

 println!("Enter a character");

    let mut input1= String::new();
    io::stdin().read_line(&mut input1)
    .expect("failed to read input");

    let ch:char=input1.trim().parse().expect("invalid input");

    if ch >= '0' && ch <= '9'{
    
        println!("Character '{ch}' is a digit ");
    }
    else {
        println!("character '{ch}' is not a digit");
    }
    
}

fn main(){
    //calling  function
    println!("Welcome! this program checks if a chatrcacter variable contains a digit or not ");

    checker()
}