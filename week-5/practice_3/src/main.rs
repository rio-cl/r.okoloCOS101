//Rust program to read the height of a person and tell if the person is tall, short or average height person

use std::io;

fn main() 
{
    let mut input = String::new();

    println!("\nEnter your height (in centimeters): ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");
    if height >= 150.0 && height <= 170.0
    {
        println!("Your are an average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You are a tall person");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are a dwarf");
    }
    else 
    {
        println!("Your height is abnormal go to a doctor ");
    }
}
