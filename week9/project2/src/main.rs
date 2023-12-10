



fn main() {
    use std::io::Write;
    use std::io;

let count = 1;

let studentname:Vec<String> = Vec::new();

        let matricnumber:Vec<String> = Vec::new();

        let studentdepartment:Vec<String> = Vec::new();

        let studentlevel:Vec<i32> = Vec::new();

    loop{

count+=1;
            print!("How many stdents ar you entering");
        let mut num = String::new();
        io::stdin().read_line(&mut num)
        .expect("Failed to read line");
        
        let num:i32 = num.trim().parse()
        .expect("Please type a number");
        
        println!("Your name please");
            let mut name =  String::new();
            io::stdin().read_line(&mut  name)
        studentname.push(name);
        
        println!("Please fill in your matric number");
            let mut matric  = String::new();
            io::stdin().read_line(&mut matric)
            .expect("input failed");
        matricnumber.push(matric);
        
        println!("Tell me your department");
            let mut department = String::new();
            io::stdin().read_line(&mut department)
            .expect("input failed");
        studentdepartment.push(department);
        
        println!("What level  are you in currently?");
            let mut level = String::new();
            io::stdin().read_line(&mut level)
            .expect("failed to input");
        
            let off_level = level.trim().parse()
            .expect("parsing failed");
        studentlevel.push(off_level);
        
        if count == num{
            break;
        } 
        }



let mut file = std::fs::File::create("PAU SMIS.xls")
.expect_err("Creation failed");

    file.write_all("                                                    PAN ATLANTIC UNIVERSITY (PAU)         STUDENT MANAGEMENT INFORMATION SYSTEM     (SMIS)".as_bytes())
    .expect("failed to write");

    
for i in 0..studentlevel.len(){
    file.write_all(studentname[i].as_bytes()).expect("failed to write");

    file.write_all(matricnumber[i].as_bytes()).expect("failed to write");

    file.write_all(studentdepartment[i].as_bytes()) .expect("failed to write");

    file.write_all(studentlevel[i].as_bytes()).expect("failed to write");
}

}
