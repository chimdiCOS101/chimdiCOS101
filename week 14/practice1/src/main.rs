use std::fs::File;
use std::io::Read;

fn main() {
    // Open the file named "staff_tb.sql"
    let mut file = File::open("staff.sql").unwrap();

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Print the contents of the file
    print!("{}", contents);
}
