fn main() {
    let fullname = " Raymond Okolo Ojonugwa";
    println!();
    println!("Name: {}", fullname);
    println!();
    println!("Before Trim");
    println!("length is {}",fullname.len());
    println!();
    println!("After Trim");
    println!("length is {}",fullname.trim().trim().len());

}
