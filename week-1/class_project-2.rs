fn main() {
	//a is toshiba
	//b is mac 
	//c is hp 
	//d is dell
	//e is acer 

	let a:f64 = 450_000.00;
	let b:f64 = 1_500_000.00;
	let c:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let e:f64 = 250_000.00;

	let s = a + b + c + d + e ;
	println!("Sum of amounts is {}", s );
    let t = 10.00;
	println!("Total quantity of products is {}", t );
	let v = s/t ;
	println!(" Average of sales record is {} ", v );

}