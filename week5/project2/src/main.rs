use std::io;

fn main() {
    println!("INCENTIVE DETERMINER FOR NEW EMPLOYEES");
    println!("\nARE YOU EXPERIANCED (1 FOR EXPERIENCED AND 2 FOR INEXPERIENCED)");

    let mut experience_rate=String::new();
    
    io::stdin().read_line(&mut experience_rate).expect("PLEASE INPUT A CORRECT VALUE BETWEEN 1 AND 2");
    let experience_rate:i32=experience_rate.trim().parse().expect("not a valid string");
    
    println!("PLEASE TELL ME YOUR AGE");
    let mut age=String::new();
    io::stdin().read_line( &mut age).expect("PLEASE ENTER VALID AGE");
    let age:i32 = age.trim().parse().expect("not a valid number");
    
        
    


    if experience_rate  == 1 {
        if age >= 40 {
             println!("YOUR INCENTIVE IS N1_560_000");
        } else if age >= 30 && age < 40 {
     
          println!("YOUR INCENTIVE IS N1_480_000");
        } else if age < 28 {
   
         println!("YOUR INCENTIVE IS N1_300_000");
        }
    } else if experience_rate == 2{
        println!("YOUR INCENTIVE IS N100_000");
    } else if experience_rate != 1|2{
        println!("PLEASE INSERT CORRECT VALUE BETWEEN 1 AND 2");
    }

    



}
