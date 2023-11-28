use std::io;
fn main() {
    let mut input1=String::new();
    
    println!("\nenter your height (in centiemeter){}",input1);
        io::stdin().read_line(&mut  input1).expect("Not a valid string");
        let height:f32=input1.trim().parse().expect("Not a valid number");
if  height >=150.0 &&  height <=170.0
{println!("You are an averaage person");}
    else if height >170.0 height <=195.0
    {println!("you are still tall");}
    else if height < 150.0 height>100.0
    {println!("DWARF");}
    else 
    {println!("abnormal height");}

}                           

