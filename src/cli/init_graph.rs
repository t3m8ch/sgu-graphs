use std::io::Write;

use crate::{
    cli::{ask_bool::ask_bool, print_graph::print_graph},
    files::load_graph,
    graph::Graph,
};

pub fn init_graph() -> Graph {
    let load_from_file = ask_bool("Загрузить граф из файла");
    if load_from_file {
        loop {
            print!("Введите путь к файлу: ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let path = input.trim();
            match load_graph(path) {
                Ok(graph) => {
                    print_graph(&graph);
                    break graph;
                }
                Err(e) => eprintln!("Ошибка загрузки графа: {}", e),
            };
        }
    } else {
        Graph::new(ask_bool("Будет ли граф ориентированным"))
    }
}
