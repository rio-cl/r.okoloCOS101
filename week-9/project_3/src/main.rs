use std::io::Write;

fn main() {
    
    let minister = vec!["Minitser                ","Aigbogun Alamba Daudu    ","Murtala Afeez Bend        ","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi    ","Osazuwa Faith Etieye    "];
    let ministry = vec!["Ministry        ","Internal affairs","Justice            ","Defense            ","Power & steel    ","Petroleum        "];
    let zone = vec!["Geo-Politicaal zone\n","South West\n","North West\n","South South\n","South West\n","South East\n"];
    let title = "****** CONVICTED MINISTERS IN NIGERIA FOR THE YEAR 2023 ******";

    let mut convicts = std::fs::File::create("Convicted miniters in Nigeria.txt").expect("Create failed");

    convicts.write_all(title.as_bytes()).expect("Write failed");

    convicts.write_all("\n".as_bytes()).expect("Write failed");
    convicts.write_all("\n".as_bytes()).expect("Write failed");


    for i in 0..minister.len(){
        convicts.write_all(minister[i].as_bytes()).expect("Write failed");
        convicts.write_all("\t|".as_bytes()).expect("Write failed");
        convicts.write_all("\t".as_bytes()).expect("Write failed");
        convicts.write_all(ministry[i].as_bytes()).expect("Write failed");
        convicts.write_all("\t|".as_bytes()).expect("Write failed");
        convicts.write_all("\t\t".as_bytes()).expect("Write failed");
        convicts.write_all(zone[i].as_bytes()).expect("Write failed");
    }

    println!("code written ");
}


