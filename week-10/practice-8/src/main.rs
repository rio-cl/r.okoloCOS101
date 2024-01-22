struct Employee {
    ceo:String,
    company:String,
    age:u32
}
fn main() {
    
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:51
    };
    let emp2 = Employee{
        company:String::from("Google Inc. "),
        ceo:String::from("Sundai Pichai"),
        age:51
    };

    display(emp1);
    
    display(emp2);
}

fn display(home:Employee){
    println!("\nName is : {}\ncompany is : {}\nAge is : {}", home.ceo,home.company,home.age);
}
