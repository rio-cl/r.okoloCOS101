fn main() {
    //Create an empty vector city
    let mut city : Vec<String> = Vec::new();
    //Print City vector
    println!("The City vector has element {}",city.len());
    //Push new elements into
    let mut input1 = String::new();
    println!("How many cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Invalid input");
    let city_num:i32 = input1.trim().parse().expect("Invaloid input");
    for count  in 0..city_num {
       let mut input2 = String::new();
       println!("Enter city {}",count+1);
       std::io::stdin().read_line(&mut input2).expect("Invalid input ");
       let new_city:String = input2.trim().parse().expect("Invalid input");
       city.push(new_city);
    }
    print!("Your preferred cities are: \n");
    let mut count=1;
    for i in city {
        println!("{} {}",count,i );
        count+=1;
    }
}
