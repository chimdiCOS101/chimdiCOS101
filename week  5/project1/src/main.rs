use std::io;
fn main() {
    println!("QUADRATIC CALCULATOR");
    println!("PLEASE INPUT YOUR VALUES");
   let mut a=String::new();
   let mut b=String::new();
   let mut c=String::new();
   io::stdin().read_line(&mut a).expect("Not  valid number");
   let mut  a:f64= a.trim().parse().expect("NOT VALID");

io::stdin().read_line(&mut b).expect("Not a valid number");
 let mut  b:f64= b.trim().parse().expect("NOT VALID");

io::stdin().read_line(&mut c).expect("Not a valid number");
let mut  c:f64= c.trim().parse().expect("NOT VALID");

let d:f64=(b*b)-(4.0*a*c);
if d == 0.0
{
    println!("one real root");
} 
else if d > 0.0
{ println!("two real root");
}
else if d < 0.0
{println!("no real root");
}
let e:f64= (-b) + d.sqrt()/(2.0*a);
println!("ONE ROOT VALUE IS {}",e );
let g:f64= (-b) - d.sqrt()/(2.0*a);
println!("SECOND ROOT VALUE IS {}",g );
}

