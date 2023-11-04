use std::io;
fn main() {
    println!("WELCOME TO SPEED CALCULATOR");
    println!("\nPLEASE FILL IN THE NECESSARY REQUIREMENTS");
    println!("\nThis is an example: A chevrolet camaro drives a distance\nof 80 miles in 2hrs, how fast does  the chevrolet go ");
    let mut d1:f64= 80.0;
    let mut  t1:f64= 2.0;
    let mut s1:f64= d1 / t1;
    println!("the speed of the chevrolet camaro is {}",s1 );
    println!("another example");
     println!("\nThis is an example: A ferrari 458 italia drives a distance\nof 120 miles in 4hrs, how fast does  the chevrolet go ");
    let  mut d2:f64= 120.0;
    let  mut t2:f64= 4.0;
    let  mut s2:f64= d2 / t2;
    println!("the speed of the ferrari is {}",s2 );
    
    println!("NOW INPUT YOUR VALUES");
   let mut d_input = String::new();
   println!("INPUT DISTANCE:");
   io::stdin().read_line(&mut d_input).expect("please enter a correct value for distance"); 
   
   let mut t_input=String::new();
    println!("INPUT TIME:");
   io::stdin().read_line(&mut t_input).expect("please enter a correct value for time");

   let mut s:f32= &d_input/&t_input;
   println!("CALCULATED SPEED IS {}",s );

}
