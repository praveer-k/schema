use serde_json::{Result, Value};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_jsonl(filename: &str) -> Result<()> {
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(data) = line {
                let v: Value = serde_json::from_str(&data)?;
                println!("Please call {} at the number {}", v["name"], v["phone"]);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    read_jsonl(filename).expect("Cannot parse json");
}
