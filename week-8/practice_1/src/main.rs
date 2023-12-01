fn main() {

    // using vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of the vector 
    println!("\nThe lenght of the Vec::new is : {}",v.len() );

    // using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    // printing the size of vector 
    println!("\nThe lenght of vec macro is {}",v.len() );
}
