use std::io::Write;

use crate::{
    files::{GraphSave, load_graph, save_graph},
    graph::{
        AddArcError, AddRibError, BaseGraph, DirectedGraph, InDegreeError,
        NodesWithGreaterOutdegreeError, OutDegreeError, RemoveArcError,
        RemoveDirectedGraphNodeError, RemoveUndirectedGraphNodeError, UndirectedGraph,
    },
};

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

fn ask_bool(msg: &str) -> bool {
    loop {
        print!("{} (yes/no): ", msg);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "yes" => break true,
            "no" => break false,
            _ => {
                println!("Некорректный ввод.");
            }
        }
    }
}

fn command_loop(mut graph: BaseGraph<i32>, directed: bool) {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().split_whitespace();

        let Some(cmd) = input.next() else {
            eprintln!("Вы должны указать команду");
            continue;
        };

        match cmd {
            "print" => print_graph(&graph),
            "clean" => {
                graph = BaseGraph::new();
                print_graph(&graph);
            }
            "add_node" => {
                let Some(value) = input.next() else {
                    eprintln!("Вы должны указать значение");
                    continue;
                };
                let Ok(value) = value.trim().parse() else {
                    eprintln!("Значение должно быть числом");
                    continue;
                };
                graph.add_node(value);
                print_graph(&graph);
            }
            "remove_node" => {
                let Some(value) = input.next() else {
                    eprintln!("Вы должны указать значение");
                    continue;
                };
                let Ok(value) = value.trim().parse() else {
                    eprintln!("Значение должно быть числом");
                    continue;
                };

                if directed {
                    let mut directed_graph: DirectedGraph<i32> = (&mut graph).into();
                    match directed_graph.remove_node(value) {
                        Ok(_) => print_graph(&graph),
                        Err(e) => match e {
                            RemoveDirectedGraphNodeError::NodeDoesNotExist => {
                                eprintln!("Узел не найден");
                                continue;
                            }
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

                                continue;
                            }
                        },
                    }
                } else {
                    let mut undirected_graph: UndirectedGraph<i32> = (&mut graph).into();
                    match undirected_graph.remove_node(value) {
                        Ok(_) => print_graph(&graph),
                        Err(e) => match e {
                            RemoveUndirectedGraphNodeError::NodeDoesNotExist => {
                                eprintln!("Узел не найден");
                                continue;
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

                                continue;
                            }
                        },
                    }
                }

                print_graph(&graph);
            }
            "add_arc" => {
                if !directed {
                    eprintln!("Граф неориентированный, добавление дуг не поддерживается");
                    continue;
                }
                let Some(from) = input.next() else {
                    eprintln!("Вы должны указать начальную вершину");
                    continue;
                };
                let Ok(from) = from.trim().parse() else {
                    eprintln!("Начальная вершина должна быть числом");
                    continue;
                };
                let Some(to) = input.next() else {
                    eprintln!("Вы должны указать конечную вершину");
                    continue;
                };
                let Ok(to) = to.trim().parse() else {
                    eprintln!("Конечная вершина должна быть числом");
                    continue;
                };

                let mut directed_graph: DirectedGraph<i32> = (&mut graph).into();
                match directed_graph.add_arc(from, to) {
                    Ok(_) => print_graph(&graph),
                    Err(e) => match e {
                        AddArcError::FromNodeDoesNotExist => {
                            eprintln!("Начальная вершина не существует")
                        }
                        AddArcError::ToNodeDoesNotExist => {
                            eprintln!("Конечная вершина не существует")
                        }
                        AddArcError::ArcAlreadyExists => eprintln!("Дуга уже существует"),
                    },
                }
            }
            "add_rib" => {
                if directed {
                    eprintln!("Граф ориентированный, добавление рёбер не поддерживается");
                    continue;
                }
                let Some(first) = input.next() else {
                    eprintln!("Вы должны указать первую вершину");
                    continue;
                };
                let Ok(first) = first.trim().parse() else {
                    eprintln!("Первая вершина должна быть числом");
                    continue;
                };
                let Some(second) = input.next() else {
                    eprintln!("Вы должны указать вторую вершину");
                    continue;
                };
                let Ok(second) = second.trim().parse() else {
                    eprintln!("Вторая вершина должна быть числом");
                    continue;
                };
                let mut undirected_graph: UndirectedGraph<i32> = (&mut graph).into();
                match undirected_graph.add_rib(first, second) {
                    Ok(_) => print_graph(&graph),
                    Err(e) => match e {
                        AddRibError::FirstNodeDoesNotExist => {
                            eprintln!("Первая вершина не существует")
                        }
                        AddRibError::SecondNodeDoesNotExist => {
                            eprintln!("Вторая вершина не существует")
                        }
                        AddRibError::RibAlreadyExists => eprintln!("Ребро уже существует"),
                    },
                }
            }
            "remove_arc" => {
                if !directed {
                    eprintln!("Граф неориентированный, удаление дуг не поддерживается");
                    continue;
                }
                let Some(from) = input.next() else {
                    eprintln!("Вы должны указать начальную вершину");
                    continue;
                };
                let Ok(from) = from.trim().parse() else {
                    eprintln!("Начальная вершина должна быть числом");
                    continue;
                };
                let Some(to) = input.next() else {
                    eprintln!("Вы должны указать конечную вершину");
                    continue;
                };
                let Ok(to) = to.trim().parse() else {
                    eprintln!("Конечная вершина должна быть числом");
                    continue;
                };
                let mut directed_graph: DirectedGraph<i32> = (&mut graph).into();
                match directed_graph.remove_arc(from, to) {
                    Ok(_) => print_graph(&graph),
                    Err(e) => match e {
                        RemoveArcError::FromNodeDoesNotExist => {
                            eprintln!("Начальная вершина не существует")
                        }
                        RemoveArcError::ToNodeDoesNotExist => {
                            eprintln!("Конечная вершина не существует")
                        }
                        RemoveArcError::ArcDoesNotExist => {
                            eprintln!("Дуги между вершинами не существует")
                        }
                    },
                }
            }
            "remove_rib" => {
                if directed {
                    eprintln!("Граф ориентированный, удаление рёбер не поддерживается");
                    continue;
                }
                let Some(first) = input.next() else {
                    eprintln!("Вы должны указать первую вершину");
                    continue;
                };
                let Ok(first) = first.trim().parse() else {
                    eprintln!("Первая вершина должна быть числом");
                    continue;
                };
                let Some(second) = input.next() else {
                    eprintln!("Вы должны указать вторую вершину");
                    continue;
                };
                let Ok(second) = second.trim().parse() else {
                    eprintln!("Вторая вершина должна быть числом");
                    continue;
                };
                let mut directed_graph: DirectedGraph<i32> = (&mut graph).into();
                match directed_graph.remove_arc(first, second) {
                    Ok(_) => print_graph(&graph),
                    Err(e) => match e {
                        RemoveArcError::FromNodeDoesNotExist => {
                            eprintln!("Первая вершина не существует")
                        }
                        RemoveArcError::ToNodeDoesNotExist => {
                            eprintln!("Вторая вершина не существует")
                        }
                        RemoveArcError::ArcDoesNotExist => {
                            eprintln!("Дуги между вершинами не существует")
                        }
                    },
                }
            }
            "out_degree" => {
                if !directed {
                    eprintln!(
                        "Граф неориентированный, получение полустепени исхода не поддерживается"
                    );
                    continue;
                }
                let Some(node_id) = input.next().and_then(|s| s.parse().ok()) else {
                    eprintln!("Вы должны указать вершину");
                    continue;
                };
                let directed_graph: DirectedGraph<i32> = (&mut graph).into();
                match directed_graph.out_degree(node_id) {
                    Ok(degree) => println!("Полустепень исхода вершины {} = {}", node_id, degree),
                    Err(e) => match e {
                        OutDegreeError::NodeDoesNotExist => {
                            eprintln!("Вершина не существует")
                        }
                    },
                }
            }
            "in_degree" => {
                if !directed {
                    eprintln!(
                        "Граф неориентированный, получение полустепени захода не поддерживается"
                    );
                    continue;
                }
                let Some(node_id) = input.next().and_then(|s| s.parse().ok()) else {
                    eprintln!("Вы должны указать вершину");
                    continue;
                };
                let directed_graph: DirectedGraph<i32> = (&mut graph).into();
                match directed_graph.in_degree(node_id) {
                    Ok(degree) => println!("Полустепень захода вершины {} = {}", node_id, degree),
                    Err(e) => match e {
                        InDegreeError::NodeDoesNotExist => {
                            eprintln!("Вершина не существует")
                        }
                    },
                }
            }
            "node_with_greater_outgree" => {
                if !directed {
                    eprintln!("Граф неориентированный, эта команда не поддерживается");
                    continue;
                }
                let Some(node_id) = input.next().and_then(|s| s.parse().ok()) else {
                    eprintln!("Вы должны указать вершину");
                    continue;
                };
                let directed_graph: DirectedGraph<i32> = (&mut graph).into();
                match directed_graph.nodes_with_greater_outdegree(node_id) {
                    Ok(nodes) => println!(
                        "Вершины, у которых полустепень захода больше, чем у {} = {:?}",
                        node_id, nodes
                    ),
                    Err(e) => match e {
                        NodesWithGreaterOutdegreeError::NodeDoesNotExist => {
                            eprintln!("Вершина не существует")
                        }
                    },
                }
            }
            "sym_diff" => {
                let Some(first_path) = input.next() else {
                    eprintln!("Вы должны указать путь для первого графа");
                    continue;
                };
                let Some(second_path) = input.next() else {
                    eprintln!("Вы должны указать путь для второго графа");
                    continue;
                };
                let Ok(mut first_graph) = load_graph(first_path) else {
                    eprintln!("Ошибка при загрузке первого графа: {}", first_path);
                    continue;
                };
                let Ok(mut second_graph) = load_graph(second_path) else {
                    eprintln!("Ошибка при загрузке второго графа: {}", second_path);
                    continue;
                };
                if !first_graph.directed {
                    eprintln!("Первый граф должен быть ориентированным");
                    continue;
                }
                if !second_graph.directed {
                    eprintln!("Второй граф должен быть ориентированным");
                    continue;
                }
                let first_graph: DirectedGraph<i32> = (&mut first_graph.graph).into();
                let second_graph: DirectedGraph<i32> = (&mut second_graph.graph).into();
                graph = first_graph.symmetric_diff(&second_graph);

                print_graph(&graph);
            }
            "save" => {
                let Some(path) = input.next() else {
                    eprintln!("Вы должны указать путь для сохранения графа");
                    continue;
                };
                if let Err(e) = save_graph(
                    &GraphSave {
                        directed,
                        graph: graph.clone(),
                    },
                    path,
                ) {
                    eprintln!("Ошибка при сохранении графа: {e}");
                    continue;
                }
                println!("Граф успешно сохранен в {}", path);
            }
            "exit" => {
                println!("Good luck with that!");
                break;
            }
            "" => eprintln!("Вы должны указать команду"),
            _ => eprintln!("Неизвестная команда"),
        }
    }
}

fn print_graph<T>(graph: &BaseGraph<T>)
where
    T: std::fmt::Debug + std::hash::Hash + Eq,
{
    println!("{graph:#?}");
}
