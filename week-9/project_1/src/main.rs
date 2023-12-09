use std::io::Write;

fn main() {
    
    let lager = vec!["33 export ","Desperado ","Goldberg ","Star \n"];
    let stout = vec!["Legend ","Turbo king ","Williams ", " \n"];
    let non_alcoholic = vec!["Maltina ","Amstel ","Malts Gold ","Fayrouz \n"];

    let mut brew = std::fs::File::create("Nigeria_Breweries_Plc.txt").expect("Create failed");
    
    for i in 0..lager.len() {
        brew.write_all(lager[i].as_bytes()).expect("write failed");
    }

    brew.write_all("\n".as_bytes()).expect("Write failed");

    for j in 0..stout.len() {
        brew.write_all(stout[j].as_bytes()).expect("write failed");
    }

    brew.write_all("\n".as_bytes()).expect("Write failed");
    
    for k in 0..non_alcoholic.len() {
        brew.write_all(non_alcoholic[k].as_bytes()).expect("write failed");
    }
    
    println!("Data writen to file");
}
