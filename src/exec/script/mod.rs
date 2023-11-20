mod pre_processer;
mod run;

use std::fs::File;
use std::io::{self, Read};

pub use run::run_entry;
pub use run::RUN;
pub use run::RUN_PATH;

pub fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    return Ok(buf);
}
