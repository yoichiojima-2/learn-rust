use std::error::Error;
use std::fs;
use std::path::Path;
use std::env;
use dotenv::dotenv;

fn read_csv(path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        data.push(record.iter().map(|x| x.to_string()).collect());
    }
    Ok(data)
}

fn search_csv_files(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new(path);
    let mut csv_files = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("csv") {
                csv_files.push(path.to_string_lossy().into_owned());
            }
        }
    } else {
        return Err(From::from("The path is not a valid directory"));
    }
    Ok(csv_files)
}

fn main() {
    dotenv().ok();
    let csv_path = env::var("CSV_PATH").expect("Failed to get CSV_PATH");
    let csv_files = search_csv_files(&csv_path).unwrap();
    for i in csv_files {
        let data = read_csv(&i).unwrap();
        // display 
        for j in data {
            println!("{:?}", j);
        }
    }
}
