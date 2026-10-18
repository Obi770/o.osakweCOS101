use std::io::{self, Write};

fn main() {
    println!("==================================================");
    println!("       EY Nigeria - Developer Recruitment         ");
    println!("==================================================\n");

    // Compound Data Type: A Vector containing Tuples of (Candidate Name, Years of Experience)
    let candidates: Vec<(&str, u32)> = vec![
        ("Adeboye", 5),
        ("Chinedu", 8),
        ("Fatima", 12),
        ("Olumide", 3),
        ("Aisha", 10),
        ("Emeka", 7),
    ];

    println!("Registered Candidates and Experience:");
    println!("------------------------------------");
    for candidate in &candidates {
        println!("Name: {}, Experience: {} years", candidate.0, candidate.1);
    }
    println!("------------------------------------\n");

    // Variables to track the highest experience found so far
    let mut highest_years: u32 = 0;
    let mut top_developer: &str = "";

    // Walk through the vector with a loop to find the maximum
    for candidate in &candidates {
        let name = candidate.0;
        let years = candidate.1;

        if years > highest_years {
            highest_years = years;
            top_developer = name;
        }
    }

    // Output the final result
    println!("==================================================");
    println!(" SELECTED CANDIDATE FOR EY NIGERIA PROJECT");
    println!("==================================================");
    println!("Name: {}", top_developer);
    println!("Years of Programming Experience: {} years", highest_years);
    println!("==================================================");
}