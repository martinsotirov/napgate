use std::fs;
use glob::glob;
use regex::Regex;
use std::io::Write;
use std::path::Path;
use std::collections::hash_set::HashSet;

fn extract_emails(emails: &mut HashSet<String>, path: &Path) {
    let re = Regex::new(r"([A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,64})").unwrap();

    if let Ok(data) = fs::read_to_string(path) {
        let lines: Vec<&str> = data.split("\n").collect();
        for line in &lines {
            if let Some(caps) = re.captures(line) {
                emails.insert(String::from(caps.get(0).unwrap().as_str()));
            }
        }
    }
}

pub fn run(path: &str, output: Option<&str>) {
    let mut emails: HashSet<String> = HashSet::new();
    if let Some(csv_path) = Path::new(path).join("**/*.csv").to_str() {
        for file in glob(csv_path).expect("Error reading csv files") {
            match file {
                Ok(path) => extract_emails(&mut emails, &path),
                Err(e) => println!("{:?}", e)
            }
        }

        println!("{} emails found", emails.len());

        if let Some(path) = output {
            let mut file = fs::File::create(path).unwrap();
            for entry in &emails {
                writeln!(file, "{}", entry).expect("Error writing to file");
            }
        }
    }
}
