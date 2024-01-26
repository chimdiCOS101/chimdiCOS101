use std::io;
use std::io::Read;
use std::fs::File;

fn asker(){
let positions = vec!["administrator","project_manager","employee","customer","vendor"];

    println!("What is your  job positiion");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
    .expect("something is  wrong");

    if input1.trim().to_lowercase() == positions[0] {
        let mut file = File::open("globacom_dbase.sql").unwrap();

        // Read the contents of the file into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        print!("{}", contents);
}
    
    
    else if input1.trim().to_lowercase() == positions[1]{
        let mut file = File::open("project.sql").unwrap();

        // Read the contents of the file into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    
    else if input1.trim().to_lowercase() == positions[2]{
        let mut file = File::open("staff.sql").unwrap();

        // Read the contents of the file into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
}
    
    
    else if input1.trim().to_lowercase() == positions[3]{
        let mut file = File::open("customer.sql").unwrap();

        // Read the contents of the file into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
}
    
    
    else if input1.trim().to_lowercase() == positions[4]{
        let mut file = File::open("dataplan.sql").unwrap();

        // Read the contents of the file into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
}
    
    
    else {
        print!("problem");
    }
}
fn main() {
    println!("                   Welcome to MTN database");
asker();
}
