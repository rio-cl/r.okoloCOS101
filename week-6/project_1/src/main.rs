use std::io;

fn main() 
{
    println!("Enter your name: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your email: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");

    println!("Enter your department: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!("Enter your State of origin: ");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Not a valid string");

    println!("Which level are you in: ");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let _level:i32 = input5.trim().parse().expect("Not a valid number");

    if _level <= 100 {
        
            println!("Sorry you are not eligible to vote ");
            return;
        
    } else if _level >= 200 && _level <= 400{
        
            println!("You Can vote");
            
    }

    println!("What is your GPA? ");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let _gpa:f32 = input6.trim().parse().expect("Not a valid number");

    if _gpa <= 4.0 
    {
        
            println!("Sorry you are not eligible to vote");
            return;
        

    }else if _gpa > 4.0 && _gpa <= 5.0
    {
        
            println!("You can vote");
           
    }

    println!("Are you currently a class rep ");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let _reps:bool = input7.to_lowercase().trim() == "yes";

    if !_reps 
    {
        println!("You are not legible to vote");
        return;
    }else {
        println!("Your can vote ");
    }

    println!("\nName : {}", input1);
    println!("Email: {}",input2 );
    println!("Department: {}",input3 );
    println!("State of orign: {}\n",input4 );






}
