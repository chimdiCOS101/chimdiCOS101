use std::io;
fn main() {
    let mut counter=0
    loop {
        
   counter+=1
    println!("                   WELCOME TO THE RESEARCHERS PUBLICATION INCENTIVE SYSTEM         ");
    println!("       OUR AIM IS TO MAKE INCENTIVE CALCULATION FOR RESEARCHERS  EASIER AND FASTER ");
    println!("Kindly tell me your name");
    let mut name=String::new();
    io::stdin().read_line(&mut name).expect("Please input your name correctly");
    println!("Welcome Mr {}",name );
    println!("--------------------------------------------------------------------------------------------------------------");
    println!("--------------------------------------------------------------------------------------------------------------");
    println!("--------------------------------------------------------------------------------------------------------------");

    println!("Currently how many papers have you published");
    let mut papers=String::new();
    io::stdin().read_line(&mut papers).expect("not a valid number");
    let papers:i32=papers.trim().parse().expect("failed to parse string");
    if papers>3 && papers <= 5 {
        println!("Congrats!! Mr {} your incentive is N 500 000", name  );
     }
     else if papers > 5 && papers <= 10{
        println!("Congrats!! Mr {} your incentive is N 800 000", name);
     }
     else if papers >= 10 {
        println!("Congrats!! Mr {} your incentive is N 1 000 000", name );
     }
     else {
         println!("sorry you have to work harder and publish more papers");
     }
     if counter==500{
        break;
     }
    }

}
