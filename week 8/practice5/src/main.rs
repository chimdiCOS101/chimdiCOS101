fn main() {

    let mut city:Vec<String>= Vec::new();

    println!("the city vector has legnth {}",city.len() );

    let mut input1 =String::new();

    println!("how many cities do you want to enter");

    std::io::stdin().read_line(&mut input1).expect("failed to read input");
let city_num:i32 = input1.trim().parse().expect("invalid input");

for count in 0..city_num{


    println!("enter city {}  count+1");

    let mut input2 = String::new();
    std::io::stdin.read_line(&mut input2).expect("failed to read  input");
let new_city:String = input2.trim().parse.expect("invalid input");

city.push(new_city);
}

    println!("your preferred city are ");

    let mut cout = 1;
     for i in city {
        println!("{} {}",count ,i );

        count +=1;
     }
}
