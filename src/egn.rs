use std::fs;
use glob::glob;
use std::io::Write;
use std::path::Path;
use std::collections::hash_set::HashSet;

fn get_egn_index(reader: &mut csv::Reader<fs::File>) -> Vec<usize> {
    let labels = ["EGN", "N_0_2_TPREPREGN", "N_0_2_TPREPREGN3", "N_0_2_GFOMAKER_EGN", "OLDEGN", "ERR_EGN",
                  "DECEGN", "WEGN", "REPRESEGN", "BSTEGN", "TINID_EGN", "WITNESSEGN", "PROXYEGN", "ID_NO",
                  "EIK", "EIK_EGN", "BULSTAT_EGN", "BS_EGN", "ZZEGN", "N_TERMREPR_EGN"];
    let mut index: Vec<usize> = Vec::new();
    for (i, header) in reader.headers().unwrap().iter().enumerate() {
        if labels.contains(&header) {
            index.push(i);
        }
    }
    index
}

fn extract_egn(egns: &mut HashSet<String>, path: &Path) {
    let mut reader = csv::Reader::from_path(path)
        .expect("Error reading csv file");

    let label_ids = get_egn_index(&mut reader);

    if !label_ids.is_empty() {
        for item in reader.records() {
            if let Ok(result) = item {
                for &label in &label_ids {
                    if let Some(egn) = result.get(label) {
                        if egn.len() == 10 {
                            egns.insert(String::from(egn));
                        }
                    }
                }
            }
        }
    }
}

pub fn run(path: &str, output: Option<&str>) {
    let mut egns: HashSet<String> = HashSet::new();
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
