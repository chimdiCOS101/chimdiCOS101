fn main() {

    let name =  vec!["mary ","sam","sally","greg","ade","mark","june","ife"];

    let age = vec![16,17,18,19,20,21,22,23];

    println!("age allocation");

    for i in 0..age.len(){

        println!("{} is {} years old",name[i],age[i] );
    }
   
}
