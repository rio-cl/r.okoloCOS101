use std::io;

fn _area_of_trapezium() {

    println!("Input height");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let _height:i32 = input2.trim().parse().expect("Invalid input");

    println!("input first base");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let _base1:i32 = input3.trim().parse().expect("Invalid input");

    println!("input second base");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Invalid input");
    let _base2:i32 = input4.trim().parse().expect("Invalid input");

    let _areat = _height/2 * (_base1 + _base2);
    println!("The area of the trapezium is : {}",_areat );

}

fn _area_of_rhombus() {
    
    println!("input diagonal 1 ");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Invalid input");
    let _diagonal1:i32 = input5.trim().parse().expect("Invalid input");

    println!("input diagonal 2 ");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6).expect("Invalid input");
    let _diagonal2:i32 = input6.trim().parse().expect("Invalid input");

    let _area_r = 1/2 * _diagonal1 * _diagonal2;
    println!("The area of the rhombus is: {}",_area_r );
}

fn _area_of_parallelogram(){
    
    println!("input base ");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("Invalid input");
    let _base:i32 = input7.trim().parse().expect("Invalid input");

    println!("input altitude ");
    let mut input8 = String::new();
    io::stdin().read_line(&mut input8).expect("Invalid input");
    let _altitude:i32 = input8.trim().parse().expect("Invalid input");

    let _areap:i32 = _base * _altitude;
    println!("The area of the Parallelogram is: {}",_areap );
}

fn _area_of_cube(){

    println!("input length of side ");
    let mut input9 = String::new();
    io::stdin().read_line(&mut input9).expect("Invalid input");
    let _side:i32 = input9.trim().parse().expect("Invalid input");

    let _areac = 6 * (_side ^ 2);
    println!("The area of the cube is: {}",_areac );
}

fn _volume_of_cylinder(){

    println!("input height ");
    let mut input10 = String::new();
    io::stdin().read_line(&mut input10).expect("Invalid input");
    let _heightcy:i32 = input10.trim().parse().expect("Invalid input");

    println!("input radius ");
    let mut input11 = String::new();
    io::stdin().read_line(&mut input11).expect("Invalid input");
    let _radius:i32 = input11.trim().parse().expect("Invalid input");

    let _volume = (22/7) * (_radius ^ 2) * (_heightcy); 
    println!("The volume of the cylinder is : {}",_volume );

}

fn main() 
{
    println!("Choose a formula.");
    println!("Area of Trapezium formula (A)");
    println!("Area of Rhombus formula(B)");
    println!("Area of a Parallelogram formula(C)");
    println!("Area of a Cube formula(D)");
    println!("Volume of Cylinder formula(E)");  

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid input");  
    let option = input1.trim();

    if option == "a" 
    {
         _area_of_trapezium()
    } else if option == "b" {
        _area_of_rhombus()
    } else if option == "c" {
        _area_of_rhombus()
    }else if option == "d" {
        _area_of_cube()
    }else if option == "e" {
        _volume_of_cylinder()
    }
}
