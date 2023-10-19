fn main() {
	//to calculate the sum, we multiply the the amount by the quantity
 let t:f64 = 450000.0 * 2.0;
 let m:f64= 1500000.0 * 1.0;
 let h:f64= 750000.0 * 3.0;
 let d:f64 = 2850000.0 * 3.0;
 let a:f64 = 250000.0 * 1.0;
 let s:f64 = t+m+h+d+a;
 println!("The sum of ammout for P.M Okeke and sons is {}", s );
 // to calculate the average, we divide the sum by the total quantity bought: 10
let avg:f64 = s/10.0;
//Therefore
println!("The averge sales for P.M Okeke and sons is {}", avg );

}
