fn main() {
	let p = 520_000_000.00;
	let r = 10.00;
	let t = 5.00;
	//compound interest
	let a = p * (1.00 + ((r / 100.00) * t));
	let cl = a - p;
	println!("Compound Interest is {}", cl);
}