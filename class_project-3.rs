fn main() {
	let p = 210_000;
	let r = 5;
	let t = 3;

	// compound interest
	let a = p * ( 1 - ( r / 100 ) ) ^ t;
	let cl = a - p;
	println!(" Amount after 3 years is {}", cl ); 
}