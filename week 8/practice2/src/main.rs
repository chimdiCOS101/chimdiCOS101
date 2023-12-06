fn main() {

    let v = Vec!['C','O','M','P','U','T','E','R'];

    let mut input1 = String::new();

    println!("Enter a value between 0 - 7" );
    std::io::stdin().read_line(&mut  input1).expect("failed to read input");

    let index:usize = input1.trim().parse().expect("invalid input");

    let ch:char = v[index];

    println!("{} is the character for index [{}]",ch,index );
    
}
