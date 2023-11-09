use std::io;
fn main() {

let  count =0;
for num in 1..21 {
    if num>10 {
        print!("{:?}",num );
        continue;
    }
    count+1;
}
println!("count of values greater than 10(between 1 and 20)is{}",count );

    
}
