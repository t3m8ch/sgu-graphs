use std::io::Write;

use crate::{
    cli::{ask_bool::ask_bool, print_graph::print_graph},
    files::{GraphSave, load_graph, save_graph},
    graph::{
        AddArcError, AddRibError, BaseGraph, DirectedGraph, InDegreeError,
        NodesWithGreaterOutdegreeError, OutDegreeError, RemoveArcError,
        RemoveDirectedGraphNodeError, RemoveUndirectedGraphNodeError, UndirectedGraph,
    },
};

pub mod cli;
pub mod files;
pub mod graph;

fn main() {
    let load_from_file = ask_bool("Загрузить граф из файла");
    let (directed, graph) = if load_from_file {
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
    };
    command_loop(graph, directed);
}

fn command_loop(mut graph: BaseGraph<i32>, directed: bool) {
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

        match dispatch_cmd(&input, &mut graph, directed) {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn dispatch_cmd(
    cmd_parts: &[String],
    mut graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    let Some(cmd) = cmd_parts.get(0) else {
        return Err("Вы должны указать команду".to_string());
    };

    match cmd.as_str() {
        "print" => Ok(print_graph(&graph)),
        "clean" => {
            *graph = BaseGraph::new();
            Ok(print_graph(&graph))
        }
        "add_node" => add_node_cmd(cmd_parts, &mut graph),
        "remove_node" => remove_node_cmd(cmd_parts, &mut graph, directed),
        "add_arc" => add_arc_cmd(cmd_parts, &mut graph, directed),
        "add_rib" => add_rib_cmd(cmd_parts, &mut graph, directed),
        "remove_arc" => remove_arc_cmd(cmd_parts, &mut graph, directed),
        "remove_rib" => remove_rib_cmd(cmd_parts, &mut graph, directed),
        "out_degree" => out_degree_cmd(cmd_parts, &mut graph, directed),
        "in_degree" => in_degree_cmd(cmd_parts, &mut graph, directed),
        "node_with_greater_outdegree" => {
            node_with_greater_outdegree_cmd(cmd_parts, &mut graph, directed)
        }
        "sym_diff" => sym_diff_cmd(cmd_parts, &mut graph, directed),
        "save" => save_cmd(cmd_parts, &mut graph, directed),
        "exit" => {
            println!("Good luck with that!");
            return Ok(false);
        }
        "" => Err("Вы должны указать команду".to_string()),
        _ => Err("Неизвестная команда".to_string()),
    }
}

fn add_node_cmd(cmd_parts: &[String], graph: &mut BaseGraph<i32>) -> Result<bool, String> {
    let Some(value) = cmd_parts.get(1) else {
        return Err("Вы должны указать значение".to_string());
    };
    let Ok(value) = value.trim().parse() else {
        return Err("Значение должно быть числом".to_string());
    };
    graph.add_node(value);
    print_graph(&graph);
    Ok(true)
}

fn remove_node_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    let Some(value) = cmd_parts.get(1) else {
        return Err("Вы должны указать значение".to_string());
    };
    let Ok(value) = value.trim().parse() else {
        return Err("Значение должно быть числом".to_string());
    };

    if directed {
        let mut directed_graph: DirectedGraph<i32> = graph.into();
        match directed_graph.remove_node(value) {
            Ok(_) => Ok(print_graph(&graph)),
            Err(e) => match e {
                RemoveDirectedGraphNodeError::NodeDoesNotExist => Err("Узел не найден".to_string()),
                RemoveDirectedGraphNodeError::NodeHasArcs { from, to } => {
                    eprintln!(
                        "Узел имеет входящие дуги с узлами {:?}, а также исходящие дуги с узлами {:?}",
                        to, from
                    );

                    if ask_bool("Удалить все дуги") {
                        for &node in &to {
                            directed_graph.remove_arc(node, value).unwrap();
                        }
                        for &node in &from {
                            directed_graph.remove_arc(value, node).unwrap();
                        }
                        directed_graph.remove_node(value).unwrap();
                        print_graph(&graph);
                    }

                    Ok(true)
                }
            },
        }
    } else {
        let mut undirected_graph: UndirectedGraph<i32> = graph.into();
        match undirected_graph.remove_node(value) {
            Ok(_) => Ok(print_graph(&graph)),
            Err(e) => match e {
                RemoveUndirectedGraphNodeError::NodeDoesNotExist => {
                    Err("Узел не найден".to_string())
                }
                RemoveUndirectedGraphNodeError::NodeHasRibs(nodes) => {
                    eprintln!("Узел имеет рёбра с узлами {:?}", nodes);

                    if ask_bool("Удалить рёбра") {
                        for node in nodes {
                            undirected_graph.remove_rib(value, node).unwrap();
                        }
                        undirected_graph.remove_node(value).unwrap();
                        print_graph(&graph);
                    }

                    Ok(true)
                }
            },
        }
    }
}

fn add_arc_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err("Граф неориентированный, добавление дуг не поддерживается".to_string());
    }
    let Some(from) = cmd_parts.get(1) else {
        return Err("Вы должны указать начальную вершину".to_string());
    };
    let Ok(from) = from.trim().parse() else {
        return Err("Начальная вершина должна быть числом".to_string());
    };
    let Some(to) = cmd_parts.get(2) else {
        return Err("Вы должны указать конечную вершину".to_string());
    };
    let Ok(to) = to.trim().parse() else {
        return Err("Конечная вершина должна быть числом".to_string());
    };

    let mut directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.add_arc(from, to) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            AddArcError::FromNodeDoesNotExist => Err("Начальная вершина не существует".to_string()),
            AddArcError::ToNodeDoesNotExist => Err("Конечная вершина не существует".to_string()),
            AddArcError::ArcAlreadyExists => Err("Дуга уже существует".to_string()),
        },
    }
}

fn add_rib_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if directed {
        return Err("Граф ориентированный, добавление рёбер не поддерживается".to_string());
    }
    let Some(first) = cmd_parts.get(1) else {
        return Err("Вы должны указать первую вершину".to_string());
    };
    let Ok(first) = first.trim().parse() else {
        return Err("Первая вершина должна быть числом".to_string());
    };
    let Some(second) = cmd_parts.get(2) else {
        return Err("Вы должны указать вторую вершину".to_string());
    };
    let Ok(second) = second.trim().parse() else {
        return Err("Вторая вершина должна быть числом".to_string());
    };
    let mut undirected_graph: UndirectedGraph<i32> = graph.into();
    match undirected_graph.add_rib(first, second) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            AddRibError::FirstNodeDoesNotExist => Err("Первая вершина не существует".to_string()),
            AddRibError::SecondNodeDoesNotExist => Err("Вторая вершина не существует".to_string()),
            AddRibError::RibAlreadyExists => Err("Ребро уже существует".to_string()),
        },
    }
}

fn remove_arc_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err("Граф неориентированный, удаление дуг не поддерживается".to_string());
    }
    let Some(from) = cmd_parts.get(1) else {
        return Err("Вы должны указать начальную вершину".to_string());
    };
    let Ok(from) = from.trim().parse() else {
        return Err("Начальная вершина должна быть числом".to_string());
    };
    let Some(to) = cmd_parts.get(2) else {
        return Err("Вы должны указать конечную вершину".to_string());
    };
    let Ok(to) = to.trim().parse() else {
        return Err("Конечная вершина должна быть числом".to_string());
    };
    let mut directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.remove_arc(from, to) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            RemoveArcError::FromNodeDoesNotExist => {
                Err("Начальная вершина не существует".to_string())
            }
            RemoveArcError::ToNodeDoesNotExist => Err("Конечная вершина не существует".to_string()),
            RemoveArcError::ArcDoesNotExist => {
                Err("Дуги между вершинами не существует".to_string())
            }
        },
    }
}

fn remove_rib_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if directed {
        return Err("Граф ориентированный, удаление рёбер не поддерживается".to_string());
    }
    let Some(first) = cmd_parts.get(1) else {
        return Err("Вы должны указать первую вершину".to_string());
    };
    let Ok(first) = first.trim().parse() else {
        return Err("Первая вершина должна быть числом".to_string());
    };
    let Some(second) = cmd_parts.get(2) else {
        return Err("Вы должны указать вторую вершину".to_string());
    };
    let Ok(second) = second.trim().parse() else {
        return Err("Вторая вершина должна быть числом".to_string());
    };
    let mut directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.remove_arc(first, second) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            RemoveArcError::FromNodeDoesNotExist => Err("Первая вершина не существует".to_string()),
            RemoveArcError::ToNodeDoesNotExist => Err("Вторая вершина не существует".to_string()),
            RemoveArcError::ArcDoesNotExist => {
                Err("Дуги между вершинами не существует".to_string())
            }
        },
    }
}

fn out_degree_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err(
            "Граф неориентированный, получение полустепени исхода не поддерживается".to_string(),
        );
    }
    let Some(node_id) = cmd_parts.get(1).and_then(|s| s.parse().ok()) else {
        return Err("Вы должны указать вершину".to_string());
    };
    let directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.out_degree(node_id) {
        Ok(degree) => {
            println!("Полустепень исхода вершины {} = {}", node_id, degree);
            Ok(true)
        }
        Err(e) => match e {
            OutDegreeError::NodeDoesNotExist => Err("Вершина не существует".to_string()),
        },
    }
}

fn in_degree_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err(
            "Граф неориентированный, получение полустепени захода не поддерживается".to_string(),
        );
    }
    let Some(node_id) = cmd_parts.get(1).and_then(|s| s.parse().ok()) else {
        return Err("Вы должны указать вершину".to_string());
    };
    let directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.in_degree(node_id) {
        Ok(degree) => {
            println!("Полустепень захода вершины {} = {}", node_id, degree);
            Ok(true)
        }
        Err(e) => match e {
            InDegreeError::NodeDoesNotExist => Err("Вершина не существует".to_string()),
        },
    }
}

fn node_with_greater_outdegree_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err("Граф неориентированный, эта команда не поддерживается".to_string());
    }
    let Some(node_id) = cmd_parts.get(1).and_then(|s| s.parse().ok()) else {
        return Err("Вы должны указать вершину".to_string());
    };
    let directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.nodes_with_greater_outdegree(node_id) {
        Ok(nodes) => {
            println!(
                "Вершины, у которых полустепень захода больше, чем у {} = {:?}",
                node_id, nodes
            );
            Ok(true)
        }
        Err(e) => match e {
            NodesWithGreaterOutdegreeError::NodeDoesNotExist => {
                Err("Вершина не существует".to_string())
            }
        },
    }
}

fn sym_diff_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err("Граф неориентированный, эта команда не поддерживается".to_string());
    }
    let Some(first_path) = cmd_parts.get(1) else {
        return Err("Вы должны указать путь для первого графа".to_string());
    };
    let Some(second_path) = cmd_parts.get(2) else {
        return Err("Вы должны указать путь для второго графа".to_string());
    };
    let Ok(mut first_graph) = load_graph(first_path) else {
        return Err(format!("Ошибка при загрузке первого графа: {}", first_path));
    };
    let Ok(mut second_graph) = load_graph(second_path) else {
        return Err(format!(
            "Ошибка при загрузке второго графа: {}",
            second_path
        ));
    };
    if !first_graph.directed {
        return Err("Первый граф должен быть ориентированным".to_string());
    }
    if !second_graph.directed {
        return Err("Второй граф должен быть ориентированным".to_string());
    }

    let first_graph: DirectedGraph<i32> = (&mut first_graph.graph).into();
    let second_graph: DirectedGraph<i32> = (&mut second_graph.graph).into();
    *graph = first_graph.symmetric_diff(&second_graph);

    Ok(print_graph(&graph))
}

fn save_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    let Some(path) = cmd_parts.get(1) else {
        return Err("Вы должны указать путь для сохранения графа".to_string());
    };
    if let Err(e) = save_graph(
        &GraphSave {
            directed,
            graph: graph.clone(),
        },
        path,
    ) {
        return Err(format!("Ошибка при сохранении графа: {e}"));
    }
    println!("Граф успешно сохранен в {}", path);
    Ok(true)
}
