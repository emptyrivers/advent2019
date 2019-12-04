
use std::env;
mod tyranny;

fn main() {
    println!("Advent Module Runner");
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => tyranny::solve(),
        _ => println!("I don't know which problem that is, sorry.")
    }
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