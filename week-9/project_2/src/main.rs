use std::io::Write;

fn main() {
    
    let title = "****** PAU-SMIS ******";
    let students = vec!["STUDENTS NAME","Oluchi Mordi","Adams Aliyu    ","Shania Bolade","Adekunle Gold","Raymond Okolo"];
    let matric_number = vec!["MATRICULATION NUMBER","ACC10211111            ","ECO10110101            ","CSC10328828            ","EEE11020202            ","SWE10202001            "];
    let department = vec!["Departments            ","Accounting            ","Economics            ","Computer Science    ","Electrical Engineering","Software Engineering"];
    let level = vec!["LEVEL\n","300\n","100\n","200\n","200\n","100\n"];

    let mut details = std::fs::File::create("PAU Student Management Information System.txt").expect("Create failed");

    details.write_all(title.as_bytes()).expect("Failed write");
    details.write_all("\n\n".as_bytes()).expect("Failed write");

    for i in 0..students.len() {
        details.write_all(students[i].as_bytes()).expect("Failed write");
        details.write_all("\t|".as_bytes()).expect("Write failed");
        details.write_all("\t".as_bytes()).expect("Write failed");
        details.write_all(matric_number[i].as_bytes()).expect("Failed write");
        details.write_all("\t|".as_bytes()).expect("Write failed");
        details.write_all("\t".as_bytes()).expect("Write failed");
        details.write_all(department[i].as_bytes()).expect("Failed write");
        details.write_all("\t|".as_bytes()).expect("Write failed");
        details.write_all("\t".as_bytes()).expect("Write failed");
        details.write_all(level[i].as_bytes()).expect("Failed write");
    }

    println!("Code written");
}
