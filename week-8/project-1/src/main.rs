fn main() {

    println!("PUBLIC SERVICE APS LEVEL CHECKER\n");


    println!("How many people do you want to check for?");
    let mut input5  = String::new();
    std::io::stdin().read_line(&mut input5).expect("Invalid input");
    let _candicate_count:i64 = input5.trim().parse().expect("Invalid input");
    
    let mut _candidate : Vec<(String,i32,String)> = Vec::new();

    for _ in 0.._candicate_count
    {

    

        println!("What is your legal name as stated in your national passport? ");
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2).expect("Invalid string");
        let _name = input2.trim().to_string();

        println!("How many yrs of experience do you have? ");
        let mut input3 = String::new();
        std::io::stdin().read_line(&mut input3).expect("Invalid input");
        let _experience:i32 = input3.trim().parse().expect("Invalid input");

        println!("What is your profession ?");
        let mut input1 = String::new();
        std::io::stdin().read_line(&mut input1).expect("Invalid input");
        let _profession = input1.trim().to_string();

        _candidate.push((_name.clone(), _experience.clone(), _profession.clone()));

        println!("What is your position under this profession");
        let mut input4 = String::new();
        std::io::stdin().read_line(&mut input4).expect("Invalid input");
        let _position = input4.trim().to_string();

            if (_experience == 2 || _experience == 1) && (_position.to_lowercase() == "intern" || _position.to_lowercase() == "paralegal" || _position.to_lowercase() == "placement") {
                println!("\nYour public service aps level is APS 1-2");
            }else if (3..=5).contains(&_experience) && (_position.to_lowercase() == "administrator" || _position.to_lowercase() == "reaserch assistant" || _position.to_lowercase() == "junior associate" || _position.to_lowercase() == "classroom teacher"){
                println!("\nYour public service aps level is APS 3-5");
            }else if (5..=8).contains(&_experience) && (_position.to_lowercase() == "senior administrator" || _position.to_lowercase() == "phd candidate" || _position.to_lowercase() == "associate" || _position.to_lowercase() == "snr teacher"){
                println!("\nYour public servis aps level is APS 5-8");
            }else if (8..=10).contains(&_experience) && (_position.to_lowercase() == "office manager" || _position.to_lowercase() == "post-doc researcher" || _position.to_lowercase() == "senior associate 1-2" || _position.to_lowercase() == "leading teacher"){
                println!("\nYour public service aps level is APS 8-10");
            }else if (10..=13).contains(&_experience) && (_position.to_lowercase() == "director" || _position.to_lowercase() == "senior lecturer" || _position.to_lowercase() == "senior associate 3-4" || _position.to_lowercase() == "deputy principal") {
                println!("\nYour public service aps level is APS 10-13");
            }else if _experience >= 13 && (_position.to_lowercase() == "ceo" || _position.to_lowercase() == "dean" || _position.to_lowercase() == "partner" || _position.to_lowercase() == "principal" ){
                println!("\nYour public service aps level is APS SES");
            }
    }
    for b in &_candidate {
        println!("Your name is: {}\nYour have {} years of expecrience\nYou profession is {}\n",b.0,b.1,b.2 );
    }

}

