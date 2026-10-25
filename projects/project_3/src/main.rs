use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Separate datasets from the cloud backup[cite: 6]
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let filename = "efcc_merged.txt";
    let mut file = File::create(filename)?;

    // Write header
    writeln!(file, "{:<30} | {:<20} | {:<15}", "Commissioner", "Ministry", "Zone")?;
    writeln!(file, "{}", "-".repeat(73))?;

    // Merge by index (Row 1 of each dataset belongs to the same person)[cite: 6]
    for i in 0..commissioners.len() {
        writeln!(
            file,
            "{:<30} | {:<20} | {:<15}",
            commissioners[i], ministries[i], zones[i]
        )?;
    }

    println!("Success! Merged records saved to '{}'.", filename);
    Ok(())
}