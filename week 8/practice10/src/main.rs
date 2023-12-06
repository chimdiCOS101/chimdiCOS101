fn print!( x:(i32,bool,f64)){
    println!("inside print method");

    let (age,is_male,cgpa)=x;


    println!("age is {}, is_male {}, cgpa is {}",age,is_male,cgpa );
}




   

}

fn main() {

     let b:(i32,bool,f64)= (30,true,4.9);
    print(b);
    

    println!("Hello, world!");
}
