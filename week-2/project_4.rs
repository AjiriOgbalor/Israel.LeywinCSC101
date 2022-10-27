fn main() {
	let t:f64 = 450_000.0;
	let m:f64 = 1_500_000.0;
	let h:f64 = 750_000.0;
	let d:f64 = 2_850_000.0;
	let a:f64 = 250_000.0;

	//Sum
	let p = (2.0 * t) + m + (3.0 * h) + (3.0 * d) + a;
	println!("Sum_of_values is {}", p);
	//Average
	let g =  p / 10.0 ;
	println!("Average_of_values is {}", g);


}