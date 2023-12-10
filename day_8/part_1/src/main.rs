use std::collections::{HashMap, VecDeque};
use std::io::stdin;

fn main() {
    let mut lines: VecDeque<String> = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0)
        .collect();
    let directions = lines
        .pop_front()
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<Vec<_>>();

    // Pop empty line
    lines.pop_front();

    // Get map of all nodes
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in &lines {
        let line_parts = parse_line(line);
        nodes.insert(line_parts.0, line_parts.1);
    }
    println!("{} nodes inserted into map", nodes.len());
    let mut current_node = "AAA";
    let mut n = 0;
    for direction in directions.iter().cycle() {
        let current_node_nexts = nodes
            .get(current_node)
            .expect(&format!("Node {} doesn't exist in map", current_node));
        if direction == &'L' {
            current_node = current_node_nexts.0;
        } else {
            current_node = current_node_nexts.1;
        }
        n += 1;
        if current_node == "ZZZ" {
            break;
        }
    }
    println!("{}", n)
}

fn parse_line(line: &String) -> (&str, (&str, &str)) {
    let parts: Vec<&str> = line.split('=').collect();
    let name = parts[0].trim();
    let directions: Vec<&str> = parts[1]
        .split(',')
        .map(|s| s.trim_matches(|c| c == '(' || c == ')' || c == ' '))
        .collect();
    (name, (directions[0], directions[1]))
}
