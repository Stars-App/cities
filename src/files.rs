use std::{
    fs::File,
    io::{self, BufReader, BufRead},
};

pub const FILES: [&str; 8] = [
    "countries/albania.csv",
    "countries/andorra.csv",
    "countries/austria.csv",
    "countries/azores.csv",
    "countries/france.csv",
    "countries/germany.csv",
    "countries/poland.csv",
    "countries/usa.csv",
];

pub fn verify() -> io::Result<()> {
    for file in FILES.iter() {
        check_integrity(file)?;
        check_duplicates(file)?;
    }

    Ok(())
}

fn check_integrity(file: &str) -> io::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.matches(';').count() != 2
            || line.starts_with(' ')
            || line.starts_with(';')
            || line.ends_with(' ')
            || line.ends_with(';')
        {
            println!("Line is not valid: {}", line);
        }
    }

    Ok(())
}

fn check_duplicates(file: &str) -> io::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if lines.contains(&line) {
            println!("Duplicate: {}", line);
        } else {
            lines.push(line);
        }
    }

    Ok(())
}
