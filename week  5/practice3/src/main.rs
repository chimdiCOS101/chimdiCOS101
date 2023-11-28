use std::io;
fn main() {
    let mut input1=String::new();
    let mut input2=String::new();
    println!("Enter base: ");
    io::stdin().read_line(&mut input1).expect("NOT A VALID STRING");
    let base:f32=input1.trim().parse().expect("not a valid number");
    io::stdin().read_line(&mut input2).expect("NOT A VALID STRING");
    let height:f32=input2.trim().parse().expect("not a valid number");

    if base>0.0 {
        let area:f32=(base*height)/2.0;
        println!("area of triangle is{}",area );
    }
}
