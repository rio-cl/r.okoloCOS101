fn main() {
    
    println!("\n**** INTERVIEW APPLICANT WITH THE HIGHEST YEARS OF EXPERIENCE SEARCHER ****\n");

    println!("\nHow interview applicants are there?\n");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Invalid input");
    let applicant_count:i64 = input1.trim().parse().expect("Invalid input");
    
    let mut _interviewe : Vec<(String,i32)> = Vec::new();

        for _ in 0..applicant_count 
        {
            println!("What is your name?");
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).expect("Invalid input");
            let _name = input2.trim().to_string();

            println!("How many years of experience do you have?");
            let mut input3 = String::new();
            std::io::stdin().read_line(&mut input3).expect("Invalid input");
            let _experience:i32 = input3.trim().parse().expect("Invalid input");

            _interviewe.push((_name.clone(), _experience));
        }
    for b  in &_interviewe {
        println!("Applicants name is {}\nApplicants has {} years of experience ",b.0,b.1 );
    }
}
