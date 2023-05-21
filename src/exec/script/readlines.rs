use std::fs::File;
use std::io::{self, BufRead};

// use crate::exec::attempt::attempt;
// use crate::public::run_time::scope::Scope;

// use super::pre_processer;

type FileBuf = io::BufReader<File>;
pub fn resolve(path: String) -> io::Result<io::Lines<FileBuf>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
