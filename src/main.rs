use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, Write};

use chrono::Local;
use log::LevelFilter;
use env_logger::{Builder, Target};
use serde_json::{Result, Value};
use indicatif::{ProgressBar, ProgressStyle};

fn read_lines<P>(filename: P) -> io::Result<(io::Lines<io::BufReader<File>>, usize)> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let mut stream = io::BufReader::new(file);
    let size = stream.fill_buf()?.len();
    Ok((stream.lines(), size))
}

fn read_jsonl(filename: &str) -> Result<()> {
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

fn main() {
    Builder::new().target(Target::Stdout).format(|buf, record| {
        writeln!(buf, 
            "{} [{}] {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    }).filter(None, LevelFilter::Info).init();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    read_jsonl(filename).expect("Cannot parse json");
}
