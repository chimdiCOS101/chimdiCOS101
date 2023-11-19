use std::io;
fn main() {
    let mut counter=1;
    loop { 
        counter+=1;
     println!("                  WELCOME TO THE PAN-ATLANTIC STUDENT VOTING SYSTEM         ");
loop {
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
    loop {
         let mut le=String::new();
        io::stdin().read_line(&mut le).expect("please input a valid level");
        let le:i32=le.trim().parse().expect("falied to parse this string");
        if le != 100 && le !=200 && le !=300 && le !=400 && le !=500 {
            println!("SORRY NOT A VALID LEVEL, PLEASE TRY AGAIN");
            continue;
        }
        if le == 100 || le ==200 || le ==300 || le ==400 || le ==500{
            break;

        }
        
    }

    println!("NAME: {}",name );
    println!("EMAIL: {}",email );
    println!("DEPARTMENT: {}",department );
    println!("STATE OF ORIGIN: {}",state_of_origin );
    println!("LEVEL: {}",le );

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
loop {
    println!("YOUR CURRENT GPA IS REQUIRED");
let mut gpa=String::new();
io::stdin().read_line(&mut gpa).expect("NOT CORRECT GPA");
let gpa:f64=gpa.trim().parse().expect("COULD NOT PARSE THIS STRING");
if gpa > 5.0 {
    println!("SORRY THAT IS NOT A VALID GPA. TRY AGAIN");
    continue;
}
 else if gpa <= 5.0 {
    break;
 }
}
println!("ARE YOU A CURRENT CLASS REPRESENTATIVE. (NOTE THE POSITION OF ASSISTANT IS NOT ALLOWED) ");
println!("PLEASE CHOOSE Y FOR YES OR N FOR NO");
let mut position=String::new();
io::stdin().read_line(&mut position).expect("please select between Y or N");
if le > 100 && le <=500 && position == "Y" && gpa > 4.0
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
