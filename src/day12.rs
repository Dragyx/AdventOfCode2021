use std::{collections::{HashMap, HashSet}, error::Error, fmt::Display, slice::SliceIndex};

use crate::helper::{load_input_for_day, out};

#[derive(Debug)]
struct Payload<'a, T> {
    value: T,
    conns: Vec<NodeID>,
    name: &'a str 
}
struct Network<'a, T> {
    nodes: HashMap<NodeID, Payload<'a, T>>,
    start: Option<NodeID>,
    end: Option<NodeID>,
    id_counter: usize
}

#[derive(Hash, Clone, Debug, PartialEq, Eq, Copy)]
pub struct NodeID(usize);

#[derive(Debug)]
struct ConnectionError {
    msg: String
}
impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ConnectionError { }

impl<'a, T> Network<'a, T> {
    pub fn add_node(&mut self, name: &'a str, node: T) -> NodeID {
        let id = NodeID(self.id_counter);
        if name == "start" {
            self.start = Some(id);
        } else if name == "end" {
            self.end = Some(id);
        }
        let payload = Payload {
            value: node,
            conns: Vec::new(),
            name,
        };
        self.nodes.insert(id, payload);
        self.id_counter += 1;
        id
    }
    pub fn connect(&mut self, id1: NodeID, id2: NodeID) -> Result<(), Box<dyn Error>> {
        if id1 == id2 {
            return Err(Box::new(ConnectionError{ msg: "Cannot connect node with itself".into() } ))
        }
        let n1 = self.nodes.get_mut(&id1).ok_or("Wrong ID")?;
        n1.conns.push(id2);
        let n2 = self.nodes.get_mut(&id2).ok_or("Wrong ID")?;
        n2.conns.push(id1);
        Ok(())
    }
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            start: None,
            end: None,
            id_counter: 0,
        }
    }
    pub fn get(&self, id: &NodeID) -> Option<&Payload<T>> {
        self.nodes.get(id)
    }
}

#[derive(Debug)]
struct Node {
    small: bool
}

struct NetworkIter<'a> {
    network: &'a Network<'a, Node>,
    path_stack: Vec<(Vec<(NodeID, &'a Payload<'a, Node>)>, bool)>,
    double_caves: bool
}
impl<'a> NetworkIter<'a> {
    pub fn new(n: &'a Network<'a, Node>, double_caves: bool) -> Self {
        let s = match n.start {
            Some(id) => vec![(vec![(id, n.get(&id).unwrap())], false)],
            None => vec![],
        };
        Self {
            network: n,
            path_stack: s,
            double_caves
        }
    } 
}

impl<'a> Iterator for NetworkIter<'a> {
    type Item = Vec<(NodeID, &'a Payload<'a, Node>)>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.network.start?;
        let end = self.network.end?;
        while let Some((unfinished_path, one_visited_twice)) = self.path_stack.pop() {
            let (last_id, node) = unfinished_path.last().unwrap();
            if *last_id == end {
                return Some(unfinished_path)
            }
            for conn in node.conns.iter() {
                let payload = self.network.get(conn).unwrap();
                // you cannot go back to the start
                if *conn == start {
                    continue
                }
                // has the cave already been visited
                let already_visited= unfinished_path 
                    .iter()
                    .any(| (id, _) | id == conn);

                if payload.value.small {
                    if self.double_caves {
                        if one_visited_twice && already_visited {
                            continue
                        }
                    } else {
                        if already_visited {
                            continue
                        }
                    }
                }
                let mut new_path = unfinished_path.clone();
                new_path.push((*conn, payload));
                self.path_stack.push((new_path, (already_visited && payload.value.small) || one_visited_twice));
            }
        }
        None
    }
}

fn print_path<'a>(p: &Vec<(NodeID, &'a Payload<'a, Node>)>) {
    for (_, payload) in p.iter().rev().skip(1).rev() {
        print!("{}", payload.name);
        print!("->");
    }
    if let Some((_, payload)) = p.last() {
        print!("{}", payload.name);
    }
    println!();
}


pub fn run() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let input = load_input_for_day(12);
    let mut network = Network::<Node>::new();
    let node_names_duplicates = input
        .lines()
        .flat_map(| line | line.split('-'));
    let mut node_names: HashSet<&str> = HashSet::new();
    for name in node_names_duplicates {
        node_names.insert(name);
    }
    let mut node_ids = HashMap::new();
    for name in node_names {
        let is_lower = name.chars().next().unwrap().is_lowercase();
        let id = network.add_node(
            name, 
            Node { 
                small: is_lower
             }
        );
        node_ids.insert(name, id);
    }
    for line in input.lines() {
        let (start, end) = line.split_once('-').unwrap();
        let idstart = node_ids.get(start).unwrap();
        let idend = node_ids.get(end).unwrap();
        network.connect(*idstart, *idend).unwrap();
    }
    let mut count_first_task: u64 = 0;
    for _p in NetworkIter::new(&network, false) {
        // print_path(&p);
        count_first_task += 1;
    }
    let mut count_second_task: u64 = 0;
    for p in NetworkIter::new(&network, true) {
        // print_path(&p);
        count_second_task += 1;
    }
    out(1)
        .var("number of paths", count_first_task)
        .print();
    out(2)
        .var("number of paths", count_second_task)
        .print();
}
