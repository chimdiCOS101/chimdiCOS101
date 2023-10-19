fn main(){
	//lets define all the variables first;
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;
	let d:f64 =  p*(1.0 - (r/100.0)).powf(n);
	//Therefore
	println!("Mrs Grace TV would be {:.2} after 3 years",d );
}