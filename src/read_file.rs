use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file(path: &str) -> Vec<(usize, usize)> {
    let file = File::open(path).expect("Could not open file");
    let buf_reader = io::BufReader::new(file);

    let mut result: Vec<(usize, usize)> = Vec::new();

    for line in buf_reader.lines() {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x = v[0].parse::<usize>().unwrap();
        // parses through edges column 1, which are the origin
        let y = v[1].parse::<usize>().unwrap();
        // parses through edges column 2, which is the destination node
        result.push((x, y));
    } 
    result
}