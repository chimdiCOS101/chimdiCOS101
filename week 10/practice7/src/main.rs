struct Employeee{
    name:String,
    company:String,
    age:u32
}

fn main() {
 let emp1 = Employeee{
    company:String::from("Ernest & Young")
    name:String::from("Ebibiong  Jessica")
    age:25
 };
 println!("Name = {} \n",emp1.name );
 println!("company = {} \n",emp1.company );
println!("age = {} \n",emp1.age );


}
