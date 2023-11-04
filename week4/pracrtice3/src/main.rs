fn main() {
let name = "chimdindu okoraFOR";
 println!("my name is {}", name );
    let name2  = name.replace("okoraFOR","john",);
    // name1 is likke a refference , so we are sayin  let name 2 be name 1 but replace the okorafor to joh
    println!("The new name is {}", name2 );
    let faculty = "faculty of SST";
    let faculty2  = faculty.replace("faculty", "of masscom");
    println!("i am a new student belongin to {}", faculty2);

}
