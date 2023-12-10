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

    // Get map of all nodes
    let mut master_node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in &lines {
        let line_parts = parse_line(line);
        master_node_map.insert(line_parts.0, line_parts.1);
    }
    println!("{} nodes inserted into map", master_node_map.len());
    let current_nodes: Vec<&str> = master_node_map
        .keys()
        .filter(|s| s.chars().last().unwrap() == 'A')
        .map(|c| *c)
        .collect();
    println!("{} matching first nodes", current_nodes.len());
    let node_multiples: Vec<usize> = current_nodes
        .iter()
        .map(|mut node| {
            let mut n: usize = 0;
            for direction in directions.iter().cycle() {
                let next_node_steps = master_node_map.get(node).unwrap();
                if direction == &'L' {
                    node = &next_node_steps.0;
                } else {
                    node = &next_node_steps.1;
                }
                n += 1;
                if node.chars().last().unwrap() == 'Z' {
                    break;
                }
            }
            n
        })
        .collect();
    let result = lcm(node_multiples.as_slice());
    println!("{}", result)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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
