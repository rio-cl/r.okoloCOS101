fn main() {
	let p = 520_000_000;
	let r = 10;
	let t = 5;

	//compound interest
	let a = p * (1 + (r / 100)^t);
	let cl = a - p;
	println!("Compound Interest is {}", cl);
}