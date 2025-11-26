use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // representing the high‑quality drink categories
    let contents = r#"High‑Quality Drink Categories

Lager:
- 33 Export
- Desperados
- Goldberg
- Gulder
- Heineken
- Star

Stout:
- Legend
- Turbo King
- Williams

Non‑Alcoholic:
- Maltina
- Amstel Malta
- Malta Gold
- Fayrouz
"#;

    // overwrite the file
    let mut file = File::create("high_quality_drinks.txt")?;

    // Write the content into the file
    file.write_all(contents.as_bytes())?;

    println!("File 'high_quality_drinks.txt' created successfully.");

    Ok(())
}
