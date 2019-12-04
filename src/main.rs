
use std::env;

// solutions
mod tyranny; // Day 1
mod opcodes; // Day 2
mod wires;   // Day 3

fn main() {
    println!("Advent Module Runner");
    let args: Vec<String> = env::args().collect();
    let nanos = match &args[1][..] {
        "1" => tyranny::solve(),
        "2" => opcodes::solve(),
        "3" => wires::solve(),
        _ => panic!("I don't know which problem that is, sorry.")
    };
    let millis = nanos / 1_000_000;
    let submillis = (nanos % 1_000_000) as f64;
    let elapsed: f64 = millis as f64 + submillis / 1_000_000 as f64;
    println!("Elapsed time: {}ms", elapsed)
}

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

pub fn get_data(path: &str) -> Vec<String> {
    let full_path = format!("./src/data/{}", path);
    let res = File::open(std::path::Path::new(&full_path));
    match res {
        Ok(f) => {
            BufReader::new(f)
                .lines()
                .map(|r| {match r { Ok(s) => s, _ => panic!("bad string")}})
                .collect()
        },
        Err(error) => {panic!("{:?} - full path: {}", error, &full_path);},
    }
}

pub fn get_data_as_ints(path: &str) -> Vec<i64> {
    let wordy_data = get_data(&path);
    wordy_data.iter().map(|s| s.parse::<i64>().unwrap()).collect()
}