struct Developer {
    name: String,
    years_of_experience: u32,
}

fn main() {
    let applicants = vec![
        Developer {
            name: String::from("Ada"),
            years_of_experience: 4,
        },
        Developer {
            name: String::from("Chinedu"),
            years_of_experience: 7,
        },
        Developer {
            name: String::from("Fatima"),
            years_of_experience: 5,
        },
        Developer {
            name: String::from("Tunde"),
            years_of_experience: 9,
        },
    ];

    let mut most_experienced = &applicants[0];

    for dev in &applicants {
        if dev.years_of_experience > most_experienced.years_of_experience {
            most_experienced = dev;
        }
    }

    println!(
        "The most experienced developer is {} with {} years of experience.",
        most_experienced.name, most_experienced.years_of_experience
    );
}
