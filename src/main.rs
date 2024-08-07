mod parsers;

use clap::Parser;

use chrono::Local;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

use parsers::jsonl::read_jsonl;

fn init_logger() {
    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "./src/mock/test.jsonl",
        help = "Specify path to the data file"
    )]
    path: String,
    #[arg(
        short,
        long,
        default_value = "jsonl",
        help = "Specify file format of the data file"
    )]
    format: String,
}

fn main() {
    init_logger();

    let args = Args::parse();

    println!("Using the following file: {}", &args.path);

    match args.format.as_str() {
        "jsonl" => {
            read_jsonl(&args.path).expect("Cannot parse the data file");
            println!("Oh! it's a JSONL !!!");
        }
        "json" => {
            println!("Oh! it's a JSON !!!");
        }
        _ => {
            println!("The format does not match any of the possible text formats");
        }
    }
}
