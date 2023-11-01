fn main() {
  let A:i32 =10;
let B:i32 =20;
println!("value of A {}", A );
println!("value of B {}", B );

let mut sol  = A>B;
println!("A is greater than B: ()",sol);

sol= A<B;
println!("A is lesser than B: ()",sol);

sol= A>=B;
println!("A is greater thanor equal to B{} ",sol);

sol= A<=B;
println!("A is lesser than or equal to B{}",sol );

sol= A==B;
println!("A is equal to B{}",sol );

sol= A!=B;
println!("A is not equal to B{}",sol );

}
