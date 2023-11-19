use std::io;

fn main() {
    let mut input1 = String::new();

    println!("enetr email");
     io::stdin().read_line(&mut input1).expect("Not a real string");
}
