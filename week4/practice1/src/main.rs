fn main() {
    let name = "aisha lawal";
    let uni:&str = "Pan Alnlantic University";
    let addr:&str = "Km 53 lagos state,ibeju lekki";
    println!("name {}", name );
    println!("university {}, \naddress: {}", uni,addr );

    let department:&'static str ="computer science";
    let  school:&'static str = "school of science and technology";
    println!("department {}, \nschool {}", department,school );


    println!("Hello, world!");
}
