use std::io;

fn main() 
{
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();

    let mut _count:i32 = 0;
    while _count < 100 
    {
    

        println!(" \nEnter patient's name: ", );
        io::stdin().read_line(&mut input1).expect("Not a real string");
        let _name = input1.trim().to_string();

        println!("Input your age: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let age:i32 = input2.trim().parse().expect("Not a valid number");

        println!("input patient's email address");
        io::stdin().read_line(&mut input3).expect("Not a real string");
        let email = input3.trim().to_string();

        println!("Input patient's phone number:");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let phone_number:i64 = input4.trim().parse().expect("Not a valid number");

        println!("How many sibling does the patient have?");
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let number_of_siblings:i32 = input5.trim().parse().expect("Not a valid number");

        println!("How many children does the patient have?");
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let children:i32 = input6.trim().parse().expect("Not a valid number");

        println!("What is patient diagnosed with?");
        io::stdin().read_line(&mut input7).expect("Not a real string");
        let _diagnosis = input7.trim().to_string();

        println!("What is patient's village of residence?");
        io::stdin().read_line(&mut input8).expect("Not a real string");
        let _place_of_residence = input8.trim().to_string();
        
        let mut amount:f32;

        if _diagnosis == "Alzheimer" && age > 50 && children > 4 && _place_of_residence.to_lowercase() == "akpabom"
        {
            
            amount = 12000000.0*0.2;
            println!("Your name is {}",input1 );
            println!("You are {} years old",age );
            println!("{}",input3 );
            println!("{}",phone_number );
            println!("You have {} number of children",children );
            println!("You are diagnosed with Alzheimer ");
            println!("You live in {} ", _place_of_residence);
            println!("Your amount is: {}",amount );
        }else if (_diagnosis == "Arrhythmia") && age == 30 && children > 4 && _place_of_residence .to_lowercase()== "ngbauji"
        {
            amount = 550000.0*0.05;
            println!("Your name is {}",input1 );
            println!("You are {} years old",age );
            println!("{}",input3 );
            println!("{}",phone_number );
            println!("You have {} number of children",children );
            println!("You are diagnosed with Arrhythmia ");
            println!("You live in {} ", _place_of_residence);

            println!("Your amount is {}",amount );
        }else if _diagnosis == "Chronic_Kidney_Disease" && age > 45 && ((children == 2) || (children == 3) || (children == 4)) && _place_of_residence.to_lowercase() ==  "atabrikang"
        {
            amount = 1500000.0*0.15;
            println!("Your name is {}",input1 );
            println!("You are {} years old",age );
            println!("{}",input3 );
            println!("{}",phone_number );
            println!("You have {} number of children",children );
            println!("You are diagnosed with Chronic_Kidney_Disease ");
            println!("You live in {} ", _place_of_residence);

            println!("Your amount is: {}",amount );
        }else if _diagnosis == "Diabetes" && age > 28 && age < 45 && ((children == 2) || (children == 3) || (children == 4))  && _place_of_residence.to_lowercase() == "okorobilom"
        {
            amount = 800000.0*0.1;
            println!("Your name is {}",input1 );
            println!("You are {} years old",age );
            println!("{}",input3 );
            println!("{}",phone_number );
            println!("You have {} number of children",children );
            println!("You are diagnosed with Diabetes ");
            println!("You live in {} ", _place_of_residence);

            println!("Your amount is: {}", amount);
        }else if _diagnosis == "Arthritis" && age > 58 && children > 5 && _place_of_residence.to_lowercase() == "emeremen" 
        {
            amount = 450000.0*0.1;
            println!("Your name is {}",input1 );
            println!("You are {} years old",age );
            println!("{}",input3 );
            println!("{}",phone_number );
            println!("You have {} number of children",children );
            println!("You are diagnosed with Arthritis ");
            println!("You live in {} ", _place_of_residence);

            println!("Your amount is: {}",amount );

        }
       _count += 1
    }    



}
