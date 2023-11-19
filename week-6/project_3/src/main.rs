use std::io;

fn main() {
    
    println!("What Number do you want to multiply ?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let _time_table:i32 = input1.trim().parse().expect("Invalid Number");

    println!("What number would you like to stop at? ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _stop:i32 = input2.trim().parse().expect("Invalid number");

    println!("{} Multiplication table", _time_table );

    let mut x = 0;
    loop {
        x += 1;
        
        let product = x * _time_table;
        println!("{} x {} = {}", x, _time_table, product );

        if x == _stop {
            break
        }
    }
}
