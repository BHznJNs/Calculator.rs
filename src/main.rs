mod compiler;
mod compute;

use std::io::{self, Write};
use compiler::compile::compile;
use compute::compute;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let tokens = compile(input);
        let result = compute(tokens);
        println!(" {}", result);
    }
}
