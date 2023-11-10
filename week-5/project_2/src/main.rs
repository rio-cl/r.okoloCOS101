use std::io;

fn main() 
{
    let mut exp = String::new();
    let mut input1 = String::new();

    let a = 1_560_000;
    let b = 1_480_000;
    let c = 1_300_000;
    let d = 100_000;

    println!("\nAre you experienced (True or False)? ");
    io::stdin().read_line(&mut exp).expect("Failed to read input");
    let experience:bool = exp.trim().parse().expect("Not valid");

    println!("How old are you?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not a valid number");

     if experience == true && age >= 40 
    {
        println!("Your incentive is {}", a);    
    }
    
      else if experience == true && age >= 30 && age < 40 
    {
        println!("Your incentive is {}", b);
    }
    
      else if experience == true && age < 28 && age > 18
      {
        println!("Your incentive is {}", c);
      }
    
     if experience == false 
      {
        println!("Your incentive is {}", d);
      }
      else if age < 18
      {
         println!("Oops sorry we dont employ people below the age of 18.\n"); 
      }

}
