




fn main() {
loop{
    println!("                                        WELCOME  TO ERNEST AND YOUNG LIMITED !            ");

    use std::io;

    let mut candidate_ages:Vec<i64>=Vec::new();
    let mut candidate_names:Vec<String>=Vec::new();
    let mut candidate_years_of_experiences:Vec<i64>=Vec::new();
    let mut candidate_contact_info:Vec<i64>=Vec::new();
    let mut martial_status:Vec<String>=Vec::new();
    let mut employment:Vec<String>=Vec::new();
    let mut occupation:Vec<String>=Vec::new();

println!("Please lets ask you some questions");

println!("What is your name?");
    let mut names =String::new();
    io::stdin().read_line(&mut names)
    .expect("Failed to read input");
candidate_names.push(names);
println!("How old are you?");    
    let mut v =String::new();
    io::stdin().read_line(&mut v)
    .expect("Failed to read input");

    let ages =v.trim().parse()
    .expect("Failed to parse");
candidate_ages.push(ages);
println!("What is your martial status");

    let mut y = String::new();
    io::stdin().read_line(&mut y)
    .expect("Failed to read input");
martial_status.push(y);
println!("Are you employed? (yes/no)");
    let mut e = String::new();
    io::stdin().read_line(&mut e)
    .expect("Failed to read input");
    let trimmed_e = e.trim();

employment.push(trimmed_e.clone().to_string());
    if trimmed_e == "yes" {
        println!("If you dont mind, what is your occupation");
        let mut  o = String::new();
        io::stdin().read_line(&mut o)
        .expect("Failed to read input");
    let trimmed_o = o.trim();

occupation.push(trimmed_o.to_string());
    }
else if trimmed_e == "no"{
    println!("Would you consider working for us");
    let mut f = String::new();
    io::stdin().read_line(&mut f)
    .expect("Failed to read input");
    let trimmed_f = f.trim();


    if trimmed_f == "yes"{
        println!("Pleas visit Ernest&young.co.org for more information to get hired");
    }

if trimmed_f == "no"{
    println!("Thank you");    }


}
else {println!("not a good response. Enter yes or no");}
 println!("Please provide us with a working telephone line");

 let  mut p = String::new();
 io::stdin().read_line(&mut p)
 .expect("Failed to read input");

 let phone:i64 =p.trim().parse()
 .expect("Not a valid telephone number");
 candidate_contact_info.push(phone);


 println!("When did you start your programming career?");
 let mut input =String::new();
 io::stdin().read_line(&mut input)
 .expect("Failed to read input");

 let start:i64 = input.trim().parse()
 .expect("Not a valid year");

let years:i64 = 2023 - start;
candidate_years_of_experiences.push(years);
println!("{} years programming! very impressive", years);
}
}
