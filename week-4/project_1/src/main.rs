use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter distance of car:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number ");

    println!("Enter time spend during the trip");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    let s:f32 = a / b ;
    let  area = s;
    

    println!(" The speed of the car is: {}km/h ", area);
}
