use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

fn main() {
    let arg = env::args()
        .skip(1)
        .collect::<Vec<String>>();
    if arg.iter().count() > 0 {
        let mut lines = 10;
        if arg.iter().count() == 2 {
            lines = arg[1].parse().unwrap_or(10);
        }
        let file = File::open(arg[0].clone()).unwrap();
        let mut reader = BufReader::new(file);
        for line in 0..lines {
            println!("{:?}", reader.by_ref().lines().next())
        }
    } else {
        println!("Incorrect number of arguments passed")
    }
}
