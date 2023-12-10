
use std::fs;
fn main() {

    fs::remove_file("data.text").expect("could not remove file")
    println!("file is removed");

}
