use crate::graph::BaseGraph;

pub fn print_graph<T>(graph: &BaseGraph<T>) -> bool
where
    T: std::fmt::Debug + std::hash::Hash + Eq,
{
    println!("{graph:#?}");
    true
}
