fn main() {
    let x = vec![100,200,300];
    borrow_vector(&x);

    println!("printing the value from main() x[0]={}",x[0] );
    println!("*************************************************");
}
fn borrow_vector(z:Vec<i32>){
    println!("****************************");
    println!("inside print_vector function {:?} \n",z );
println!("_______________________________________________________");

}