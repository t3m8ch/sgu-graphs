use std::io::Write;

use crate::{
    cli::{ask_bool::ask_bool, print_graph::print_graph},
    files::load_graph,
    graph::BaseGraph,
};

pub fn init_graph() -> (bool, BaseGraph<i32>) {
    let load_from_file = ask_bool("Загрузить граф из файла");
    if load_from_file {
        loop {
            print!("Введите путь к файлу: ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let path = input.trim();
            match load_graph(path) {
                Ok(graph_save) => {
                    print_graph(&graph_save.graph);
                    break (graph_save.directed, graph_save.graph);
                }
                Err(e) => eprintln!("Ошибка загрузки графа: {}", e),
            };
        }
    } else {
        (
            ask_bool("Будет ли граф ориентированным"),
            BaseGraph::<i32>::new(),
        )
    }
}
