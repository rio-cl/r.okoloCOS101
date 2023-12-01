fn main() {
    
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}",mountain_heights );

    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}",mountain_heights );

    mountain_heights.0 = "Universe";
    mountain_heights.1 = 4563;

    println!("Double chnaged = {:?}",mountain_heights );
}
