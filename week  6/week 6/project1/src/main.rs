
use std::io;
fn trapezium(a: f64, b:f64, c:f64){

    let trapezium= a/2.0 * (b+c);
    println!("AREA OF THE TRAPEZIUM IS {trapezium}");

}

fn rhombus(d:f64, e:f64){
    let rhombus = (d*e)/2.0;
    println!("THE AREA OF THE RHOMBUS IS {rhombus}");

} 

fn parallelogram(f:f64, g:f64){

    let  calc= f*g;
    println!("THE AREA OF THE PARALLELOGRAM IS {calc}");

}
fn cube(h:f64){
    let  calc= 6.0 * (h*h);
    println!("THE AREA OF THE CUBE IS {calc}");


}
fn cylider(i:f64, j:f64){
    let calc= 3.142 * (i*i)* j;
    println!("THE AREA OF THE CYLINDER IS {calc}");


}
fn main() {
    loop{
    println!("                                               WELCOME TO CHIMDIS CALCULATOR                ");

    println!("I CAN CALCULATE THE AREA OF TRAPEZIUMS, RHOMBUS, CUBES, PARALLELOGRAM, AND  CYLINDERS");
println!(" WHAT WOULD YOU WANT ME TO CALCULATE FOR YOU.   
    IF TRAPEZIUM SELECT 1

    IF RHOMBUS SELECT 2
    
    IF PARALLELOGRAM SELECT 3

    IF CUBE SELECT 4

    IF CYLINDER SELECT 5");

        let  mut input1=String::new();
        io::stdin().read_line(&mut input1)
        .expect("not a valid option");
        
        let selection :i32=input1.trim().parse()
        .expect("failed to parse string");
    
        if selection != 1 && selection  > 5 {
            println!("PLEASE SELECT A VALID OPTION ");
        continue;
}
            
            
                    
      
        
    
        
        

if selection == 1 {
    println!("TRAPEZIUM CALCULATOR ACTIVATED");

    println!("ENTER HEIGHT");
    let mut input2=String::new();
    io::stdin().read_line(&mut input2)
    .expect("not a valid height");

let a:f64=input2.trim().parse()
.expect("failed to parse string");

println!("ENTER LENGTH OF FIRST BASE");
let mut input3=String::new();
io::stdin().read_line(&mut input3)
.expect("not a valid  base length ");

let b:f64=input3.trim().parse()
.expect("failed to parse string");

println!("ENTER LENGTH OF SECOND BASE");
let mut input4=String::new();
io::stdin().read_line(&mut input4)
.expect("not a valid base legnth");

let c:f64=input4.trim().parse()
.expect("failed to parse string");  

trapezium(a, b, c);
break;
}

else if selection == 2{
    println!("                   RHOMBUS CALCULATOR ACTIVATED          ");

println!("ENTER THE LENGTH OF THE FIRST  DIAGONAL");
    let mut input5= String::new();
    io::stdin().read_line(&mut input5)
    .expect("not a valid diagonal length");

    let d:f64=input5.trim().parse()
    .expect("failed to parse string");


    println!("ENTER THE LENGTH OF THE SECOND  DIAGONAL");
    let mut input6= String::new();
    io::stdin().read_line(&mut input6)
    .expect("not a valid diagonal length");

    let e:f64=input6.trim().parse()
    .expect("failed to parse string");
rhombus(d, e);
break;

}
else if selection == 3{
    println!("                               PARALLELOGRAM CALCULATOR ACTIVATED      ");
    println!("ENTER BASE");

    let mut input7= String::new();
    io::stdin().read_line(&mut input7)
    .expect("not a valid base length");

    let f:f64=input7.trim().parse()
    .expect("failed to parse string");


    println!("ENTER THE ALTITUDE");
    let mut input8= String::new();
    io::stdin().read_line(&mut input8)
    .expect("not a valid  altitude");

    let g:f64=input8.trim().parse()
    .expect("failed to parse string");
parallelogram(f, g);
break;
}

else if selection ==4{
    println!("                               CALCULATOR ACTIVATED      ");
    println!("ENTER SIDE LENGTH");

    let mut input9= String::new();
    io::stdin().read_line(&mut input9)
    .expect("not a valid base length");

    let h:f64=input9.trim().parse()
    .expect("failed to parse string");
cube(h);
break;

}

else if selection == 5{

   println!("                               PARALLELOGRAM CALCULATOR ACTIVATED      ");
    println!("ENTER RADIUS LENGTH");

    let mut input10= String::new();
    io::stdin().read_line(&mut input10)
    .expect("not a valid radius length");

    let i :f64=input10.trim().parse()
    .expect("failed to parse string");


    println!("ENTER THE HEIGHT");
    let mut input11= String::new();
    io::stdin().read_line(&mut input11)
    .expect("not a valid height");

    let j :f64=input11.trim().parse()
    .expect("failed to parse string");
cylider(i, j);
break;
}
else {
    println!("sorry i do not understand your request");
    continue;
}
                           }
}
