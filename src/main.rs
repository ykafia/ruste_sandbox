use daggy::{
    Dag,
    WouldCycle,
    NodeIndex
};
use std::iter::once;

struct Weight;

fn add_edges_ok() {
    let mut dag = Dag::<Weight, u32, u32>::new();
    let root = dag.add_node(Weight);
    let a = dag.add_node(Weight);
    let b = dag.add_node(Weight);
    let c = dag.add_node(Weight);

    let edges = once((root, a, 0))
        .chain(once((root, b, 1)))
        .chain(once((root, c, 2)));
    let mut new_edges = dag.add_edges(edges).unwrap();

    assert_eq!(new_edges.next(), dag.find_edge(root, a));
    assert_eq!(new_edges.next(), dag.find_edge(root, b));
    assert_eq!(new_edges.next(), dag.find_edge(root, c));
}


fn main() {
    println!("Hello, world!");
    add_edges_ok();
    let x = Batman{};
    is_he_good(x);
    let rule = Rule::new(
        |us| {
            match us<5 {
                true=> true,
                false => false
            }
        }
    );
    println!("result is {}", (rule.pm)(5));

}


trait IsGood {}

struct Superman;
struct Batman;

impl IsGood for Superman{}
impl IsGood for Batman{}

fn is_he_good(hero : impl IsGood){
    println!("Yes {} is good", print_type_of(&hero))
}
fn print_type_of<T>(_: &T) -> String{
    format!("{}", std::any::type_name::<T>())
}

struct Rule<T>{
    pm : fn(T) -> bool
}

impl Rule<usize> {
    fn new(rule : fn(usize) -> bool) -> Rule<usize> {
        Rule::<usize> {
            pm : rule
        }
    }
}