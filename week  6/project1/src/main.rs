use std::io;
fn main() {
    let mut counter=1;
    loop { 
        counter+=1;
     println!("                  WELCOME TO THE PAN-ATLANTIC STUDENT VOTING SYSTEM         ");

    println!("                                USER DETAILS                                ");
    println!("PLEASE TELL US YOUR NAME");
    let mut name=String::new();
    io::stdin().read_line(&mut name).expect("please input a valid name");

    println!("WE  WOULD LIKE TO KNOW YOUR EAMIL IN ORDER TO CONTACT YOU");
    let mut email=String::new();
    io::stdin().read_line(&mut email).expect("please input a valid email address");

    println!("WHAT DEPARTMENT DO YOU STUDY IN ?");
    let mut department=String::new();
    io::stdin().read_line(&mut department).expect("please input a valid department in the school");

    println!("WHAT IS YOUR STATE OF ORIGIN ?");
    let  mut state_of_origin=String::new();
    io::stdin().read_line(&mut state_of_origin).expect("please input a valid state in The Federal republic of Nigeria");
 
    println!("WHAT IS YOUR CURRENT LEVEL IN {}", department);
         let mut level=String::new();
        io::stdin().read_line(&mut level).expect("please input a valid level");
        let level:i32=level.trim().parse().expect("falied to parse this string");
        if level != 100 && level !=200 && level !=300 && level !=400 && level !=500 {
            println!("SORRY NOT A VALID LEVEL, PLEASE TRY AGAIN");
            break; 

    println!("NAME: {}",name );
    println!("EMAIL: {}",email );
    println!("DEPARTMENT: {}",department );
    println!("STATE OF ORIGIN: {}",state_of_origin );
    println!("LEVEL: {}",level );

println!("IS THE ABOVE INFORMATION ABOUT YOU CORRECT ? PLEASE SELECT Y/N (YES OR NO)");
let  mut choice=String::new();
io::stdin().read_line(&mut choice).expect("PLEASE SELECT BETWEEN Y/N (YES OR NO)");
if choice== "Y" {
    break;

}
 else if choice =="N"{

        println!("WE ARE SORRY, HERE IS ANOTHER CHANCE");
        continue;
    }
}
println!("ONWARD!!!!!!!!!!!");

    println!("YOUR CURRENT GPA IS REQUIRED");
let mut gpa=String::new();
io::stdin().read_line(&mut gpa).expect("NOT CORRECT GPA");
let gpa:f64=gpa.trim().parse().expect("COULD NOT PARSE THIS STRING");
if gpa > 5.0 {
    println!("SORRY THAT IS NOT A VALID GPA. TRY AGAIN");
    break;
}
 
}
println!("ARE YOU A CURRENT CLASS REPRESENTATIVE. (NOTE THE POSITION OF ASSISTANT IS NOT ALLOWED) ");
println!("PLEASE CHOOSE Y FOR YES OR N FOR NO");
let mut position=String::new();
io::stdin().read_line(&mut position).expect("please select between Y or N");
if level > 100 && level <=500 && position == "Y" && gpa > 4.0
{
    println!("CONGRATS YOU ARE ELIGIBLE TO VOTE");
    println!("                                            {}",name );
    println!("EMAIL: {}",email);
    println!("DEPARTMENT: {}",department );
    println!("STATE_OF_ORIGIN:{}",state_of_origin);
}
else {
    println!("SORRY YOU ARE NOT ELIGIBLE TO VOTE");
}



 if counter == 150{
    break;
 } 
   }
    

}
    println!("Hello, world!");
}
