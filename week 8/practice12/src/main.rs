fn main() {

let mut colors=["red","green","yellow","white"];
println!("original copy of array is {}",colors );

let sliced_colors = &mut colors[1..3];
println!("first slice  is {:?}", sliced_colors );

sliced_colors[1]= "purple";

println!("changed slice is {}",sliced_colors );


    println!("Hello, world!");
}
