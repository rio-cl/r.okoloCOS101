 struct Laptop {
    hp:i32,
    ibm:i32,
    toshiba:i32,
    dell:i32
}

impl Laptop {
    fn sum(&self)->i32{
        (self.hp * 3) + (self.ibm * 3) + (self.toshiba * 3) + (self.dell * 3)
    }
}

fn main() {
    
    let prices = Laptop {
        hp:650_000,
        ibm:755_000,
        toshiba:550_000,
        dell:850_000
    };
    println!("Price of 3 HP laptops is {}", prices.hp * 3);
    println!("Price of 3 IBM laptops is {}", prices.ibm * 3);
    println!("Price of 3 Toshiba laptops is {}", prices.toshiba * 3);
    println!("Price of 3 Dell laptops is {}\n", prices.dell * 3);
    println!("Summ of all the laptops purchased is {}\n", prices.sum());
}