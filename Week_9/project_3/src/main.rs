use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Dataset 1: Names of Commissioners
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical Zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Display merged records on screen
    println!("EFCC – Consolidated Records of Convicted Ministers\n");

    for i in 0..names.len() {
        println!("Record {}", i + 1);
        println!("Name: {}", names[i]);
        println!("Ministry: {}", ministries[i]);
        println!("Geopolitical Zone: {}", zones[i]);
        println!("----------------------------------------");
    }

    // Prepare formatted output for file
    let mut output = String::new();

    output.push_str("EFCC – Consolidated Records of Convicted Ministers\n\n");
    output.push_str("S/N   NAME OF COMMISSIONER                 MINISTRY             GEOPOLITICAL ZONE\n");
    output.push_str("-----------------------------------------------------------------------------------\n");

    for i in 0..names.len() {
        output.push_str(&format!(
            "{}     {:30} {:20} {}\n",
            i + 1,
            names[i],
            ministries[i],
            zones[i]
        ));
    }

    // Save to file
    let mut file = File::create("efcc_convicted_ministers.txt")?;
    file.write_all(output.as_bytes())?;

    println!("\nFile 'efcc_convicted_ministers.txt' created successfully.");

    Ok(())
}

