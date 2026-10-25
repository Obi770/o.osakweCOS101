use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("================================================================================");
    println!("                     PAU Student Management Information System                  ");
    println!("================================================================================\n");

    // 1. Define the student data structure using a vector of tuples:
    // (Student Name, Matric. Number, Department, Level)
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    // 2. Display records on the screen
    println!("{:<20} | {:<15} | {:<15} | {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    println!("{}", "-".repeat(63));

    for student in &students {
        println!(
            "{:<20} | {:<15} | {:<15} | {:<5}",
            student.0, student.1, student.2, student.3
        );
    }

    // 3. Save records into a file
    let filename = "pau_smis_records.txt";
    let mut file = File::create(filename)?;

    writeln!(file, "PAU - STUDENT MANAGEMENT INFORMATION SYSTEM (SMIS)\n")?;
    writeln!(file, "{:<20} | {:<15} | {:<15} | {:<5}", "Student Name", "Matric. Number", "Department", "Level")?;
    writeln!(file, "{}", "-".repeat(63))?;

    for student in &students {
        writeln!(
            file,
            "{:<20} | {:<15} | {:<15} | {:<5}",
            student.0, student.1, student.2, student.3
        )?;
    }

    println!("\n[Success] Student details have been successfully saved to '{}'.", filename);

    Ok(())
}