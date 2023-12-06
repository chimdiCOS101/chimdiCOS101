fn value(n:option<&char>){

    println!("  element of vector {:?}",n );
}

fn main() {
   
let v = Vec!['R','U','S','T','A','C','I','A','N',];

let mut input1=String::new();
println!("enter a value between 0 - 8");

std::io::stdin().read_line(&mut input1).expect("failed to read input")

let index:usize = input1.trim().parse().expect("invalid input")


let ch:  option<&char> = v.get(index);

value(ch);

println!("{:?}", );


}
