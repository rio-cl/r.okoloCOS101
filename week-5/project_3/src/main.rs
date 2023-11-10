use std::io;

fn main() 
{
    let p = "Poundo Yam/ Edinkaiko Soup";
    let f = "Fried Rice & Chicken";
    let a = "Amala & Ewedu Soup"; 
    let e = "Eba & Egusi Soup";
    let w = "White Rice & Stew";
    let mut input1 = String::new();

    println!("Menu: \n{} = N3,200.0 \n{} = N3,000.0 \n{} = N2,500.0 \n{} = N2,000.0 \n{} = N2,500.0\n", p, f, a, e, w);

    println!("Input your price: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let price:f32 = input1.trim().parse().expect("Not a valid number");

    if price == 3200.0 
    {
        println!("Your order is: {}", p);
    }
    else if price == 3000.0
    {
        println!("Your order is: {}", f);
    }
    else if price == 2500.0 
    {
        println!("Your order is {} or {}",a , w);
    }
    else if price ==  2000.0 
    {
        println!("Your order is {}", e);
    }

    if price > 10000.0
    {
        let discount = price - ( 0.05 * price );
        println!("Your discount is {}", discount );
    }

}
