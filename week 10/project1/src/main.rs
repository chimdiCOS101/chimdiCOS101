struct Electronics {
laptop_brand:String,
quantity:i32,
total:i32,
}
impl Electronics{
    fn reciept(&self)->i32{
        if self.laptop_brand == "HP"{
            self.quantity*650000
        }
        else if  self.laptop_brand == "DELL"{
            self.quantity*850000
}
          else if self.laptop_brand == "IBM"{
            self.quantity*755000
        }
         else if self.laptop_brand == "Toshiba"{
            self.quantity*550000
        }
        else {
        println!("Null request");
       return 0
        }
    }
}

fn sum(quantity:i32, laptop_brand:String)->i32{
if laptop_brand == "HP"{
    quantity*650000
}
else if laptop_brand == "Dell"{
    quantity*850000
}
else if laptop_brand == "Toshiba"{
    quantity*550000
}
else if laptop_brand == "IBM"{
    quantity*755000
}
else {
   println!("Null request");
   return 0
}
}

fn main() {
    use std::io;
    use std::fs::File;
    use std::io::Write;

    loop{


let customer_laptops:Vec<String> = Vec::new();

let customer_quantity:Vec<i32> = Vec::new();

let customer_total:Vec<i32> = Vec::new();


    println!("                                          WELCOME TO MR OGBEIFUNA'S ELECTRONICS STORE ");

println!("What is the name of the laptop brand you want to buy?");



let mut insert = String::new();
io::stdin().read_line(&mut insert)
.expect("Failed to read input");
customer_laptops.push(insert);

let laptop_brand=insert.trim();

println!("how many {} laptops do you want to buy?",laptop_brand );

let mut input = String::new();
io::stdin().read_line(&mut input)
.expect("Failed to read user input");

 let quantity:i32 = input.trim().parse()
.expect("Parsing failed");

customer_quantity.push(quantity);


let total = sum(quantity, laptop_brand.to_string());


let mut file =File::create("MR OGBEIFUNA'S LOG BOOK.txt")
.expect("create failed");

file.write_all(  "                                                 MR OGBEIFUNA'S LOG BOOK OF 2023              ".as_bytes());

file.write_all("\nLAPTOP                                \tQUANTITY                                          \tTOTAL".as_bytes());

for i in 0..customer_laptops.len(){

    file.write_all(customer_laptops[i].as_bytes()).expect("Write failed");
    file.write_all(b",").expect("Failed to write");

    file.write_all(customer_quantity[i].as_bytes()).expect("Write failed");
    file.write_all(b",").expect("Failed to write");

    file.write_all(customer_total[i].as_bytes()).expect("Write failed");
    file.write_all(b",").expect("Failed to write");

    file.write_all(b"\n").expect("Failed to write");
}

let customer = Electronics{

laptop_brand: laptop_brand.to_string,
quantity,
total,

};

println!("Would you like to buy another laptop? (Y--yes , N--no)");
let mut  response = String::new();
io::stdin().read_line(&mut response)
.expect("Please choose well next time");

let response = response.trim();

if response == "Y"{
    continue;
}
else if response == "N"{
println!("Your total is {}. Thank you for shopping at MR OGBEIFUNA'S ELECTRONICS STORE ",customer.reciept());

    break;

}


}
}