use std::io::{self, Write};

// Define a struct to represent a row in the public service table
struct PublicServant {
    aps_level: String,
    office_admin: String,
    academic: String,
    lawyer: String,
    teacher: String,
}

fn main() {
    println!("==================================================");
    println!("    MTH/CSC 101: APS Level Validation System     ");
    println!("==================================================\n");

    // Initialize a Vector to store our dataset (Table from assignment)
    let mut dataset: Vec<PublicServant> = Vec::new();

    // Populate the dataset using standard vector push methods
    dataset.push(PublicServant {
        aps_level: String::from("APS 1-2"),
        office_admin: String::from("Intern"),
        academic: String::from("-"),
        lawyer: String::from("Paralegal"),
        teacher: String::from("Placement"),
    });

    dataset.push(PublicServant {
        aps_level: String::from("APS 3-5"),
        office_admin: String::from("Administrator"),
        academic: String::from("Research Assistant"),
        lawyer: String::from("Junior Associate"),
        teacher: String::from("Classroom Teacher"),
    });

    dataset.push(PublicServant {
        aps_level: String::from("APS 5-8"),
        office_admin: String::from("Senior Administrator"),
        academic: String::from("PhD Candidate"),
        lawyer: String::from("Associate"),
        teacher: String::from("Senior Teacher"),
    });

    dataset.push(PublicServant {
        aps_level: String::from("EL1 8-10"),
        office_admin: String::from("Office Manager"),
        academic: String::from("Post-Doc Researcher"),
        lawyer: String::from("Senior Associate 1-2"),
        teacher: String::from("Leading Teacher"),
    });

    dataset.push(PublicServant {
        aps_level: String::from("EL2 10-13"),
        office_admin: String::from("Director"),
        academic: String::from("Senior Lecturer"),
        lawyer: String::from("Senior Associate 3-4"),
        teacher: String::from("Deputy Principal"),
    });

    dataset.push(PublicServant {
        aps_level: String::from("SES"),
        office_admin: String::from("CEO"),
        academic: String::from("Dean"),
        lawyer: String::from("Partner"),
        teacher: String::from("Principal"),
    });

    // Display Category Menu
    println!("Select Profession Track:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
    print!("\nEnter option (1-4): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");
    let choice = choice.trim();

    // Prompt for Job Title
    print!("Enter Job Title (e.g., Associate, Director, Senior Teacher): ");
    io::stdout().flush().unwrap();

    let mut user_title = String::new();
    io::stdin().read_line(&mut user_title).expect("Failed to read title");
    let user_title = user_title.trim();

    // Search and validation loop
    let mut is_found = false;

    for row in &dataset {
        // Select which column to inspect based on user's choice
        let matched_role = match choice {
            "1" => &row.office_admin,
            "2" => &row.academic,
            "3" => &row.lawyer,
            "4" => &row.teacher,
            _ => "",
        };

        // Check for match (case-insensitive conversion)
        if matched_role.to_lowercase() == user_title.to_lowercase() {
            println!("\n--------------------------------------------------");
            println!(" RESULT FOUND:");
            println!(" Position: {}", matched_role);
            println!(" APS Level: {}", row.aps_level);
            println!("--------------------------------------------------");
            is_found = true;
            break;
        }
    }

    if !is_found {
        println!("\nError: Position '{}' not found for the selected category.", user_title);
    }
}