use std::io;
use std::fs::File;
use std::io::Write;

struct Company {
    name: String,
    founded: i32,
    shares: f64,
    liabilities: f64,
}

impl Company {
    fn leverage_percentage(&self) -> f64 {
        ((self.shares - self.liabilities) / self.shares) * 100.0
    }

    fn leverages_used(&self) -> f64 {
        if self.liabilities < 10_000_000.00 {
            ((self.shares - self.liabilities) / self.shares) * 100.0
        } else {
            0.0
        }
    }
}

fn main() {
    let companies = [
        Company {
            name: String::from("Cadbury Nigeria Plc"),
            founded: 1965,
            shares: 15_000_000.00,
            liabilities: 5_500_000.00,
        },
        Company {
            name: String::from("Champion Breweries"),
            founded: 1974,
            shares: 25_000_000.00,
            liabilities: 8_000_000.00,
        },
        Company {
            name: String::from("Dangote Sugar Refinery Plc"),
            founded: 1970,
            shares: 18_000_000.00,
            liabilities: 10_000_000.00,
        },
        Company {
            name: String::from("Flour Mills Nigeria Pic"),
            founded: 1960,
            shares: 32_000_000.00,
            liabilities: 4_000_000.00,
        },
        Company {
            name: String::from("Nestle Nigeria panic"),
            founded: 1961,
            shares: 8_000_000.00,
            liabilities: 1_500_000.00,
        },
        Company {
            name: String::from("Unilever Nigeria PIc"),
            founded: 1923,
            shares: 37_000_000.00,
            liabilities: 11_000_000.00,
        },
        Company {
            name: String::from("Honeywell Nigeria PIc"),
            founded: 1906,
            shares: 34_000_000.00,
            liabilities: 9_000_000.00,
        },
        Company {
            name: String::from("Nigerian Breweries Plc"),
            founded: 1946,
            shares: 30_000_000.00,
            liabilities: 12_000_000.00,
        },
    ];

    let title = "********** SPRINGATE MODEL DATABASE **********";
    let titles =
        "Company                     | Year Founded  |        Shares     |  Liabilities      | Percentage Leverages  | 5 percent of percentage leverages\n";

    let mut details =
        File::create("Springate_model_database.txt").expect("Create failed");

    writeln!(details, "{}\n\n\n{}", title, titles).expect("Failed writing");

    writeln!(
        details,
        "----------------------------|---------------|-------------------|-------------------|-----------------------|--------------------------------"
    )
    .expect("Failed writing");

    for company in &companies {
        writeln!(
            details,
            "{:<30} | {:>13} | {:>17} | {:>17} | {:>23.2} % | {:>32.2} %",
            company.name.trim(),
            company.founded,
            company.shares,
            company.liabilities,
            company.leverage_percentage(),
            company.leverages_used()
        )
        .expect("Failed writing");
    }

    println!("Enter your username");
    let username: String = read_line_trim();

    let password: String;
    match username.to_lowercase().as_str() {
        "cadb" => password = String::from("cadb1"),
        "cham" => password = String::from("cham2"),
        "dang" => password = String::from("dang3"),
        "flou" => password = String::from("flou4"),
        "nest" => password = String::from("nest5"),
        "unil" => password = String::from("unil6"),
        "hone" => password = String::from("hone7"),
        "nige" => password = String::from("nige8"),
        _ => {
            println!("Invalid username");
            return;
        }
    }

    println!("Enter password");
    let user_input_password: String = read_line_trim();

    if user_input_password == password {
        // Continue with file operations
        let _details = File::open("Springate_model_database.txt").unwrap();
    } else {
        println!("Incorrect password");
    }
}

fn read_line_trim() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    input.trim().to_string()
}