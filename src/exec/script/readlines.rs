use std::fs::File;
use std::io::{self, BufRead};

type FileBuf = io::BufReader<File>;
pub fn resolve(path: &str) -> io::Result<io::Lines<FileBuf>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
