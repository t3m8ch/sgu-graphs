use std::io::Write;

use crate::graph::{
    AddArcError, AddRibError, BaseGraph, DirectedGraph, RemoveArcError,
    RemoveDirectedGraphNodeError, RemoveUndirectedGraphNodeError, UndirectedGraph,
};

pub mod graph;

fn main() {
    let directed = ask_bool("Будет ли граф ориентированным");
    let graph: BaseGraph<i32> = BaseGraph::new();
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
