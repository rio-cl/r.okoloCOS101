fn main() {
     let x = vec![100,200,300];
     borrow_vector(&x);

     println!("Prinnting value from main() x[0]={}",x[0] );
 }

fn borrow_vector(z:&Vec<i32>){

    println!("****************************");
    println!("Inside print_vector function {:?} \n", z);
    println!("-----------------------------");
}
