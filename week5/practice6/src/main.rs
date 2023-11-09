use std::io;

fn main() {
        let mut input1=String::new();
        let mut input2=String::new();
        println!("enter lower bound");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let lower_bound:i32=input1.trim().parse().expect("Failed to input");

        println!("Enter upper bound");
        io::stdin().read_line(&mut input2).expect("not a valid string");
        let upper_bound:i32=input2.trim().parse().expect("Failed to input")

        for x in lower_bound..upper_bound{println!("Count level is {}",x );}

}
