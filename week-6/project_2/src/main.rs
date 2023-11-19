use std::io;

fn main() 
{
    let mut count:i32 = 0;
    while count < 500 
    {
    


    println!("Enter your name: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid string");

    println!("How many papers have you published ?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid String");
    let _papers:i32 = input2.trim().parse().expect("Invalid Number");

    if _papers >= 3 && _papers <= 5 {
        println!("\nName: {}
            Your incentive is N500,000\n",input1 );
    }else if _papers > 5 && _papers < 10{
        println!("\nName: {}
            Your incentive is N800,000\n",input1 );
    }else if _papers >= 10 {
        println!("\nName: {}
            Your incentive is N1,000,000\n",input1 );
    }else if _papers < 3 {
        println!("\nName: {}
            Your incentive is N100,000\n",input1 );
    }
    count += 1
    }



}
