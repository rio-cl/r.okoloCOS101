use std::io;

fn siblings_details()
{

    println!("Siblings first name");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let _name = input2.trim();

    println!("How old is the sibling");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let _age:i32 = input3.trim().parse().expect("Invalid input");

    if _age > 18 
    {
        println!("is sibling married or working");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Invalid input");
        let _marrital_status = input4.trim();
        
        if  _marrital_status == "single"{
            println!("is sibling a student or worker");
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("Invalid input");
            let _working_status = input5.trim();

            if _working_status == "student" {
                println!("Which universty does the attend");
                let mut input6 = String::new();
                io::stdin().read_line(&mut input6).expect("Invalid input");
                let _university = input6.trim();
                println!("What course did this sibling study");
                let mut input7 = String::new();
                io::stdin().read_line(&mut input7).expect("Invalid input");
                let _course = input7.trim();
            }
        }else if _marrital_status == "married" {
            println!("Does sibling have any offspring");
            let mut input8 = String::new();
            io::stdin().read_line(&mut input8).expect("Invalid input");
            let _offspring = input8.trim();

            println!("Where city does this siblings family live in");
            let mut input9 = String::new();
            io::stdin().read_line(&mut input9).expect("Invalid input");
            let _residency = input9.trim();
        }
    }else if _age < 18 {
        println!("Has this sibling written waec");
        let mut input10 = String::new();
        io::stdin().read_line(&mut input10).expect("Invalid input");
        let _waec_status:bool = input10.to_lowercase().trim() == "yes";

        if _waec_status {
            println!("What secondary school did this sibling attend ?");
            let mut input11 = String::new();
            io::stdin().read_line(&mut input11).expect("Invalid input");
            let _secondary_school = input11.trim();
        }else if !_waec_status {
            println!("What is this sibling current class ?");
            let mut input12 = String::new();
            io::stdin().read_line(&mut input12).expect("Invalid input");
            let _class = input12.trim();
        }
    }

    println!("NAME : {}",_name );
    println!("AGE : {}",_age );

    if _age > 18 
    {
            
            println!("Marrital status{}", _marrital_status);
            if _marrital_status == "married" 
            {
                println!("Number of offspring: {}",_offspring );
                println!("City og family residency: {}",_residency);
            }else if _marrital_status == "single" 
            {
                println!("working status: {}",_working_status);
                if _working_status == student 
                {
                    println!("UNIVERSITY : {}",_university );
                    println!("COURSE OF STUDY: {}",_course );
                }
            }      

        }else if _age < 18 {
            println!("WAEC STATUS: {}",_waec_status );
            if _waec_status 
            {
                println!("SECONDARY SCHOOL ATTENDED: {}",_secondary_school );
            }else if !_waec_status {
                println!("CURRENT CLASS LEVEL: {}",_class );
            }
    }
}

fn main() {
    println!("How many siblings do you have");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let _siblings:i32 = input1.trim().parse().expect("Invalid input");


    let mut count:i32 = 0;
    while count < _siblings   {

        siblings_details();
         count += 1
    }
    
}
