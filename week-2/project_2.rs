fn main() {
	//Toshiba
	let qty1:f64 = 2.0; 
	let amt_1:f64 = 450_000.0;
	let total_1:f64 = qty1 * amt_1;

	//Mac
	let qty2:f64 = 1.0;
	let amt_2:f64 = 1_500_000.0;
	let total_2:f64 = qty2 * amt_2;

	//HP
	let qty3:f64 = 3.0;
	let amt_3:f64 = 750_000.0;
	let total_3:f64 = qty3 * amt_3;

	//Dell
	let qty4:f64 = 3.0;
	let amt_4:f64 = 2_850_000.0;
	let total_4:f64 = qty4 * amt_4;

	//Acer
	let qty5:f64 = 1.0;
	let amt_5:f64 = 250_000.0; 
	let total_5 = qty5 * amt_5; 

	//Sum
	let sum = total_1 + total_2 + total_3 + total_4 + total_5;
	println!("Total amount of sales is {}", sum);

	//Average
	let total_qty = qty1 + qty2 + qty3 + qty4 + qty5;
	let average = sum / total_qty;
	println!("The average sale is {}", average);

	}
