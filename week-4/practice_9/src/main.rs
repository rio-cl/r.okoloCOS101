fn main() {
   let a:i32 = 10;
   let b:i32 = 20;

   println!("\nValue of A:{} ",a);
   println!("Value of B:{}",b);

   let mut res = a>b ;
   println!("A greater than B: {}",res );

   res = a<b ;
   println!("A is less than B: {}",res );

   res = a>=b ;
   println!("A greater than or equal to B: {} ",res );

   res = a<=b ;
   println!("a lesser than or equal to b: {}",res );

   res = a!=b ;
   println!("a is not equal to b: {} \n",res );
   
}
