use std::io;

fn main() 
{ 
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Input coefficient A: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Failed to read input");

    println!("Input coefficient B: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Failed to read input");

    println!("Input coefficient C: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:i32 = input3.trim().parse().expect("Failed to read input");

    let discriminant = (b*b) - (4*a*c);

    if discriminant > 0  
    {
        println!("Equation has exactly 2 real distinct roots");
    }
    else if discriminant == 0 
    {
        println!("Equation has exactly 1 real root");
    }
    else if discriminant < 0
    {
        println!("Equation has no real roots");
    }
}
