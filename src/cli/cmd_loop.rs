use std::io::Write;

use crate::{cli::dispatch_cmd::dispatch_cmd, graph::Graph};

pub fn cmd_loop(mut graph: Graph) {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        match dispatch_cmd(&input, &mut graph) {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => eprintln!("{e}"),
        }
    }
}
