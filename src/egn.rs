use std::fs;
use glob::glob;
use std::io::Write;
use std::path::Path;
use std::collections::hash_set::HashSet;

fn get_egn_index(reader: &mut csv::Reader<fs::File>) -> Option<usize> {
    let mut index: Option<usize> = None;
    for (i, header) in reader.headers().unwrap().iter().enumerate() {
        if header == "EGN" {
            index = Some(i);
            break;
        }
    }
    index
}

fn extract_egn(egns: &mut HashSet<i64>, path: &Path) {
    let mut reader = csv::Reader::from_path(path)
        .expect("Error reading csv file");

    if let Some(index) = get_egn_index(&mut reader) {
        for item in reader.records() {
            if let Ok(result) = item {
                if let Some(egn) = result.get(index) {
                    if let Ok(egn_int) = egn.parse::<i64>() {
                        egns.insert(egn_int);
                    }
                }
            }
        }
    }
}


pub fn run(path: &str, output: Option<&str>) {
    let mut egns: HashSet<i64> = HashSet::new();
    if let Some(csv_path) = Path::new(path).join("**/*.csv").to_str() {
        for file in glob(csv_path).expect("Error reading csv files") {
            match file {
                Ok(path) => extract_egn(&mut egns, &path),
                Err(e) => println!("{:?}", e)
            }
        }

        println!("{} EGNs found", egns.len());

        if let Some(path) = output {
            let mut file = fs::File::create(path).unwrap();
            for egn in &egns {
                writeln!(file, "{}", egn).expect("Error writing EGN to file");
            }
        }
    }
}
