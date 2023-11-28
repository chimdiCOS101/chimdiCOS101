use std::io;
fn main() {
println!("            WELCOME TO MR GOOD FOODS AND SONS            ");
println!("FOOD MENU ");
println!("MENU                                               PRICE(per portion)
p =     Poundo Yam/Edinkaiko Soup                              - N3,200
f =     Fried Rice & Chicken                                   - N3,000
a =     Amala & Ewedu Soup                                     - N2,500
e =     Eba & Egusi Soup                                       - N2,000
w =     White Rice & Stew                                      - N2,500
\nPROMO! PROMOOO! PROMOOOOOOO! GET AN ORDER ABOVE 10_000 AND GET 5% FREEEEEE
");

let p:i32= 3_200;
let f:i32= 3_000;
let a:i32= 2_500;
let e:i32= 2_000;
let w:i32= 2_500;
loop{
println!("FIRST ORDER");
let mut input1=String::new();
io::stdin().read_line(&mut input1).expect("Please input a value on the menu.Thank you");
let input1:i32= input1.trim().expect("not a valid string");

if input1 == "p" {println!("You chose Poundo Yam"); 
} 
else if input1 == "f" { println!("You chose Fried Rice & Chicken");
 } 
else if input1 == "a" { println!("You chose Amala & Ewedu Soup");
} 
else if input1 == "e" { println!("You chose Eba & Egusi Soup");
} 
else if input1 == "w" { println!("You chose White Rice & Stew");
} 
else { println!("Invalid choice. Please select from the menu.");continue;
} 

let mut confirm_order=String::new();
io::stdin().read_line(&mut confirm_order).expect("not a valid option");
let confirm_order:i32= confirm_order.trim().expect("not a valid string");
             if confirm_order == p {println!("You have confirmed that your prefered meal is  Poundo Yam"); } 
else if confirm_order == "f" { println!("You  have confirmed that your prefered meal is  Fried Rice & Chicken"); } 
else if confirm_order == "a" { println!("You  have confirmed that your prefered meal is  Amala & Ewedu Soup");} 
else if confirm_order == "e" { println!("You  have confirmed that your prefered meal is  Eba & Egusi Soup");} 
else if confirm_order == "w" { println!("You  have confirmed that your prefered meal is  White Rice & Stew"); } 
else if confirm_order == "k" {println!("You  have confirmed that your prefered meal is  nothing");} 
else { println!("Invalid choice. Please select from the menu.");} 
}
}

// println!("SECOND ORDER");
// let mut input2=String::new();
// io::stdin().read_line(&mut input2).expect("Please input a value on the menu.Thank you");
// let input2:i32=input2.trim().expect("not a valid string");

//              if input2 == "p" {println!("You chose Poundo Yam"); } 
// else if input2 == "f" { println!("You chose Fried Rice & Chicken"); } 
// else if input2 == "a" { println!("You chose Amala & Ewedu Soup");} 
// else if input2 == "e" { println!("You chose Eba & Egusi Soup");} 
// else if input2 == "w" { println!("You chose White Rice & Stew"); } 
// else if input2 == "k" {println!("You chose nothing");} 
// else { println!("Invalid choice. Please select from the menu.");} 
// if input2 != "p" && input2 !="f" && input2 !="a" && input2 !="e" && input2 !="w" && input2 !="k"
// loop {
//     println!("{}",input2 );
// }
// let mut confirm_order=String::new();
// io::stdin().read_line(&mut confirm_order).expect("not a valid option");
// let confirm_order:i32= confirm_order.trim().expect("not a valid string");
//              if confirm_order == "p" {println!("You have confirmed that your prefered meal is  Poundo Yam"); } 
// else if confirm_order == "f" { println!("You  have confirmed that your prefered meal is  Fried Rice & Chicken"); } 
// else if confirm_order == "a" { println!("You  have confirmed that your prefered meal is  Amala & Ewedu Soup");} 
// else if confirm_order == "e" { println!("You  have confirmed that your prefered meal is  Eba & Egusi Soup");} 
// else if confirm_order == "w" { println!("You  have confirmed that your prefered meal is  White Rice & Stew"); } 
// else if confirm_order == "k" {println!("You  have confirmed that your prefered meal is  nothing");} 
// else { println!("Invalid choice. Please select from the menu.");} 
// if confirm_order != "p" && confirm_order !="f" && confirm_order !="a" && confirm_order !="e" && confirm_order !="w" && confirm_order !="k" 
// loop {
//     println!("{}",confirm_order );
// }

// println!("THIRD ORDER");
// let mut input3=String::new();
// io::stdin().read_line(&mut input3).expect("Please input a value on the menu.Thank you");
// let input3:i32=input3.trim().expect("not a valid string");  

//              if input3 == "p" {println!("You chose Poundo Yam"); } 
// else if input3 == "f" { println!("You chose Fried Rice & Chicken"); } 
// else if input3 == "a" { println!("You chose Amala & Ewedu Soup");} 
// else if input3 == "e" { println!("You chose Eba & Egusi Soup");} 
// else if input3 == "w" { println!("You chose White Rice & Stew"); } 
// else if input3 == "k" {println!("You chose nothing");} 
// else { println!("Invalid choice. Please select from the menu.");} 
// if input2 != "p" && input2 !="f" && input2 !="a" && input2 !="e" && input2 !="w" && input2 !="k"
// loop {
//     println!("{}",input2 );
// }
// let mut confirm_order=String::new();
// io::stdin().read_line(&mut confirm_order).expect("not a valid option");
// let confirm_order:i32= confirm_order.trim().expect("not a valid string");
//              if confirm_order == "p" {println!("You have confirmed that your prefered meal is  Poundo Yam"); } 
// else if confirm_order == "f" { println!("You  have confirmed that your prefered meal is  Fried Rice & Chicken"); } 
// else if confirm_order == "a" { println!("You  have confirmed that your prefered meal is  Amala & Ewedu Soup");} 
// else if confirm_order == "e" { println!("You  have confirmed that your prefered meal is  Eba & Egusi Soup");} 
// else if confirm_order == "w" { println!("You  have confirmed that your prefered meal is  White Rice & Stew"); } 
// else if confirm_order == "k" {println!("You  have confirmed that your prefered meal is  nothing");} 
// else { println!("Invalid choice. Please select from the menu.");} 
// if confirm_order != "p" && confirm_order !="f" && confirm_order !="a" && confirm_order !="e" && confirm_order !="w" && confirm_order !="k" 
// loop {
//     println!("{}",confirm_order );
// }
 
// println!("FOURTH ORDER");
// let mut input4=String::new();
// io::stdin().read_line(&mut input4).expect("Please input a value on the menu.Thank you");
// let input4:i32=input4.trim().expect("not a valid string");   

//              if input4 == "p" {println!("You chose Poundo Yam"); } 
// else if input4 == "f" { println!("You chose Fried Rice & Chicken"); } 
// else if input4 == "a" { println!("You chose Amala & Ewedu Soup");} 
// else if input4 == "e" { println!("You chose Eba & Egusi Soup");} 
// else if input4 == "w" { println!("You chose White Rice & Stew"); } 
// else if input4 == "k" {println!("You chose nothing");} 
// else { println!("Invalid choice. Please select from the menu.");} 
// if input2 != "p" && input2 !="f" && input2 !="a" && input2 !="e" && input2 !="w" && input2 !="k"
// loop {
//     println!("{}",input2 );
// }
// let mut confirm_order=String::new();
// io::stdin().read_line(&mut confirm_order).expect("not a valid option");
// let confirm_order:i32= confirm_order.trim().expect("not a valid string");

// if confirm_order == p {println!("You have confirmed that your prefered meal is  Poundo Yam");
// } 
// else if confirm_order == "f" { println!("You  have confirmed that your prefered meal is  Fried Rice & Chicken"); 
// } 
// else if confirm_order == "a" { println!("You  have confirmed that your prefered meal is  Amala & Ewedu Soup");
// } 
// else if confirm_order == "e" { println!("You  have confirmed that your prefered meal is  Eba & Egusi Soup");
// } 
// else if confirm_order == "w" { println!("You  have confirmed that your prefered meal is  White Rice & Stew");
// } 
// else if confirm_order == "k" {println!("You  have confirmed that your prefered meal is  nothing");
// } 
// else { println!("Invalid choice. Please select from the menu.");
// } 
// if confirm_order != 1 && confirm_order !=2 && confirm_order !=3 && confirm_order !=4 && confirm_order !=5 && confirm_order !=6 
// loop {
//     println!("{}",confirm_order );
// }



// println!("FIFTH ORDER");
// let mut input5=String::new();
// io::stdin().read_line(&mut input5).expect("Please input a value on the menu.Thank you");
// let input5:i32=input5.trim().expect("not a valid string"); 

//              if input5 == "p" {println!("You chose Poundo Yam"); } 
// else if input5 == "f" { println!("You chose Fried Rice & Chicken"); } 
// else if input5 == "a" { println!("You chose Amala & Ewedu Soup");} 
// else if input5 == "e" { println!("You chose Eba & Egusi Soup");} 
// else if input5 == "w" { println!("You chose White Rice & Stew"); } 
// else if input5 == "k" {println!("You chose nothing");} 
// else { println!("Invalid choice. Please select from the menu.");} 
// let mut confirm_order=String::new();
// io::stdin().read_line(&mut confirm_order).expect("not a valid option");
// let confirm_order:i32= confirm_order.trim().expect("not a valid string");
//              if confirm_order == p {println!("You have confirmed that your prefered meal is  Poundo Yam"); } 
// else if confirm_order == "f" { println!("You  have confirmed that your prefered meal is  Fried Rice & Chicken"); } 
// else if confirm_order == "a" { println!("You  have confirmed that your prefered meal is  Amala & Ewedu Soup");} 
// else if confirm_order == "e" { println!("You  have confirmed that your prefered meal is  Eba & Egusi Soup");} 
// else if confirm_order == "w" { println!("You  have confirmed that your prefered meal is  White Rice & Stew"); } 
// else if confirm_order == "k" {println!("You  have confirmed that your prefered meal is  nothing");} 
// else { println!("Invalid choice. Please select from the menu.");} 
// if confirm_order != 1 && confirm_order !=2 && confirm_order !=3 && confirm_order !=4 && confirm_order !=5 && confirm_order !=6 
// loop {
//     println!("{}",confirm_order );
// }


