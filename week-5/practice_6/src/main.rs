use std::io;

fn main() {
    println!("Enter a number: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut x:i32 = input1.trim().parse().expect("Failed to read input") ;

    //while true

    loop {
        x+=1;
        println!("x={}",x);

    if x==15 {
        break;
      }
    }
}
