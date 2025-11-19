fn main() {
    let roles = vec![
        ("APS 1-2", vec![
            ("Office Administrator", "Intern"),
            ("Academic", "Research Assistant"),
            ("Lawyer", "Public Associate"),
            ("Teacher", "Classroom Teacher"),
        ]),
        ("APS 3-5", vec![
            ("Office Administrator", "Senior Administrator"),
            ("Academic", "PhD Candidate"),
            ("Lawyer", "Senior Associate 1-2"),
            ("Teacher", "Snr Teacher"),
        ]),
        ("APS 5-8", vec![
            ("Office Administrator", "Office Manager"),
            ("Academic", "Post-Doc Researcher"),
            ("Lawyer", "Senior Associate 3-4"),
            ("Teacher", "Deputy Principal"),
        ]),
        ("EL1-EL2", vec![
            ("Office Administrator", "Director"),
            ("Academic", "Senior Lecturer"),
            ("Lawyer", "Partner"),
            ("Teacher", "Principal"),
        ]),
        ("SES", vec![
            ("Office Administrator", "CEO"),
            ("Academic", "Dean"),
            ("Lawyer", "Head"),
            ("Teacher", "Principal"),
        ]),
    ];

    let profession = "Lawyer";
    let role = "Senior Associate 3-4";
    let experience = 6;

    let mut level = "Unknown";

    for (aps_level, mappings) in &roles {
        for (prof, title) in mappings {
            if *prof == profession && *title == role {
                level = aps_level;
            }
        }
    }

    // Additional validation rule
    if profession == "Lawyer" && role.contains("Associate") && (5..=8).contains(&experience) {
        level = "APS 5-8";
    }

    println!("The staff level is: {}", level);
}
