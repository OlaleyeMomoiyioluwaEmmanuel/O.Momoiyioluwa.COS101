use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Student data stored in vectors
    let names = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matric_numbers = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001",
    ];

    let departments = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let levels = vec!["300", "100", "200", "200", "100"];

    // Display the details on screen
    println!("PAU SMIS");
    println!("Student Records\n");

    for i in 0..names.len() {
        println!("Name: {}", names[i]);
        println!("Matric Number: {}", matric_numbers[i]);
        println!("Department: {}", departments[i]);
        println!("Level: {}", levels[i]);
        println!("-----------------------------");
    }

    // Prepare formatted output for file
    let mut output = String::new();

    output.push_str("Student Name\n");
    output.push_str("Oluchi Mordi\nAdams Aliyu\nShania Bolade\n\nB\n\n");

    output.push_str("PAU SMIS\n");
    output.push_str("Matric. Number   Department   Level\n");

    for i in 0..names.len() {
        output.push_str(&format!(
            "{}   {}   {}\n",
            matric_numbers[i], departments[i], levels[i]
        ));
    }

    // Save to file
    let mut file = File::create("pau_smis_records.txt")?;
    file.write_all(output.as_bytes())?;

    println!("\nFile 'pau_smis_records.txt' created successfully.");

    Ok(())
}
