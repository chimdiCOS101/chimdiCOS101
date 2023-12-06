



fn main() {


    println!("                                         WELCOME TO THE PUBLIC SERVICE APS LEVEL CHECKER ");
    println!("                                          A PRODUCT OF THE FEDRAL GOVERNMENT OF NIGERIA    ");



    println!("Hello there.May  i know your name? :]");
    let mut  name= String::new();
    std::io::stdin().read_line(&mut name)
    .expect("Not a valid name");



    println!("When did you start working here as a staff?");

let  mut year = String::new();
std::io::stdin().read_line(&mut year)
.expect("not a valid year ");

let year_started:i32 =year.trim().parse()
.expect("Failed  to parse string");

let years_of_experience = 2023 - year_started;

if year_started == 2023 {
    println!("We are very sorry but you are too young to be assigned a position :[");
}
 
else if year_started < 2023 {
     
     println!("You have been in the industry for {} years! Very  impressive {}", years_of_experience,name);
            
}



let office_administrator=vec!["intern","Administrator","Senior administrator","Office manager","Director","CEO"];

let academic=vec!["Uncertain","Research Assistant","PhD candidate","Post-doc Researcher","Senior Lecturer","Dean"];

let lawyer=vec!["paralegel","Junior assistant","Associate","Senior Associate (1-2)","senior Associate (3-4)","Partner"];

let teacher=vec!["Placement","Classroom Teacher","Senior teacher","Leading teacher","Deputy principal","principal"];

let public_servant=vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];



println!("Please select your job title");
println!("1= Office Administrator,  2= Academic,  3= Lawyer,  4= Teacher");

let mut input=String::new();
std::io::stdin().read_line(&mut input)
.expect("not a valid job in this organization");

let job_input:i32=input.trim().parse()
.expect("Failed to parse string");

if job_input == 1{

    println!("  what position do you hold as an office administrator?

               INTERN = 1

               ADMINISTRATOR = 2

               SENIOR ADMINISTRATOR = 3

               OFFICE MANAGER = 4

               DIRECTOR = 5

               CEO = 6
               ");
}

else if job_input == 2 {
    println!("what position do you hold in the academic sector?

               NO POSITION = 1

              RESEARCH ASSISTANT = 2

              PHD CANDIDATE = 3

              POST-DOC RESEARCHER = 4

              SENIOR LECTURER = 5

               DEAN = 6");
}

else if job_input == 3{
    println!("what position do you hold as a lawyer?

               PARALEGAL = 1

               JUNIOR ASSISTANT = 2

               ASSOCIATE = 3

               SENIOR ASSOCIATE (1-2) = 4

               SENIOR ASSOCIATE (3-4) = 5

               PARTNER= 6");
}

else if job_input == 4{
println!("what position do you hold  a teacher?

               INTERN = 1

               ADMINISTRATOR = 2

               SENIOR ADMINISTRATOR = 3

               OFFICE MANAGER = 4

               DIRECTOR = 5

               CEO = 6")

}
else {
    
    println!("Sorry i dont understand your request");
}
 




let mut input2=String::new();
std::io::stdin().read_line(&mut input2)
.expect("not a valid job in this organization");

let job_position:i16=input2.trim().parse()
.expect("Failed to parse string");


let count=1;


// if job_input == 1 {

    println!("You hold the position of {}",public_servant[(job_position -1) as usize] );


//    if job_position == 1{
//     println!("You hold the position of {}",public_servant[i] );
//    }
//    else if job_position == 2{
//     println!("You hold the position of {}",public_servant[i] );
//    }
//    else if job_position == 3{
//     println!("You hold the position of {}",public_servant[i] );
//    }
//    else if job_position == 4{
//     println!("You hold the position of {}",public_servant[i] );
//    }
// else {
// println!("sorry i dont understand your request");
// }              }


// 


//     if job_input == 2{

//          println!("You hold the position of {}",public_servant[(job_position -1) as usize] );

// // if job_position == 1{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 2{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 3{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 4{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// // else {
// // println!("sorry i dont understand your request"); }

             
// }






//     if job_input == 3{

//          println!("You hold the position of {}",public_servant[(job_position -1) as usize] );

// // if job_position == 1{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 2{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 3{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 4{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// // else {
// // println!("sorry i dont understand your request"); }

             


// }



//     if job_input == 4{

//          println!("You hold the position of {}",public_servant[(job_position -1) as usize] );

// // if job_position == 1{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 2{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 3{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// //    else if job_position == 4{
// //     println!("You hold the position of {}",public_servant[i] );
// //    }
// // else {
// // println!("sorry i dont understand your request"); }   

             
// 



}