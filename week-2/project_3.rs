fn main() {
	let p:f64 = 510_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//Depreciation
	let z = (1.0 -(r / 100.0)).powf(n);
	let a = p * z;
		println!("The value of the TV after 3 years is {}", a);
}