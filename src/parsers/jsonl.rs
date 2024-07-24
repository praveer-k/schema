use std::fs::File;
use std::path::Path;

use std::io::{self, BufRead};
use serde_json::{Result, Value};
use indicatif::{ProgressBar, ProgressStyle};

pub fn read_lines<P>(filename: P) -> io::Result<(io::Lines<io::BufReader<File>>, usize)> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let mut stream = io::BufReader::new(file);
    let size = stream.fill_buf()?.len();
    Ok((stream.lines(), size))
}

pub fn read_jsonl(filename: &str) -> Result<()> {
    if let Ok((lines, size)) = read_lines(filename) {
        let bar = ProgressBar::new(size as u64);
        bar.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.green}] ({pos}/{len}, ETA {eta})"));
        for line in lines {
            bar.inc(1);
            if let Ok(data) = line {
                log::info!("{}", format!("{}", data));
                let v: Value = serde_json::from_str(&data)?;
                bar.set_message(format!("{}",v["name"]));
            }
        }
        bar.finish_with_message("read all lines");
    }
    Ok(())
}
