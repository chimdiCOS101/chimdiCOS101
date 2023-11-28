//the iter() function fetches all elements ina an array
fn main() {
    let arr:[i32;4]=[10,20,30,40];

    println!("array is {:?}",arr);
    println!("array length is {}",arr.len());

    for val in arr.iter(){
        println!("value  is {}",val);
    }
    
}
