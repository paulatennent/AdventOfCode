use crate::opt::{Opt, Question};
use regex::Regex;
use std::collections::{HashMap};

pub fn solve(opt: Opt, input: String) {
    match opt.question {
        Question::A => {
            println!("Solution to A: {}", get_a(input));
        }
        Question::B => {
            println!("Solution to B: {}", get_b(input));
        }
    }
}

fn get_a(input: String) -> i64 {
    let (path, mapping) = input.split_once("\n\n").unwrap();
    let path = path.chars().collect::<Vec<_>>();
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    re.captures_iter(mapping).for_each(|line| {
        let loc = line.get(1).unwrap().as_str();
        let left = line.get(2).unwrap().as_str();
        let right = line.get(3).unwrap().as_str();
        graph.insert(loc, (left, right));
    });

    get_distance(&graph, &path, "AAA", Some("ZZZ"))
}

fn get_distance(
    graph: &HashMap<&str, (&str, &str)>,
    path: &Vec<char>,
    start: &str,
    end: Option<&str>,
) -> i64 {
    let mut counter = 0;
    let mut curr = start;
    let path_len = path.len();
    let mut indices = (0..path_len).cycle();
    loop {
        let dir = path[indices.next().unwrap()];
        let entry = graph.get(curr).unwrap();
        match dir {
            'L' => curr = entry.0,
            'R' => curr = entry.1,
            _ => (),
        }
        counter += 1;
        match end {
            Some(end) => {
                if curr == end {
                    break;
                }
            }
            None => {
                if curr.ends_with('Z') {
                    break;
                }
            }
        };
    }
    counter
}

fn get_b(input: String) -> i64 {
    let (path, mapping) = input.split_once("\n\n").unwrap();
    let path = path.chars().collect::<Vec<_>>();
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    re.captures_iter(mapping).for_each(|line| {
        let loc = line.get(1).unwrap().as_str();
        let left = line.get(2).unwrap().as_str();
        let right = line.get(3).unwrap().as_str();
        graph.insert(loc, (left, right));
    });

    let starts = graph.keys().clone().filter(|node| node.ends_with('A'));
    starts.clone().for_each(|node| {
        let dist = get_distance(&graph, &path, node, None);
        println!("{:?}, {:?}", node, dist);
    });
    starts
        .map(|node| get_distance(&graph, &path, node, None))
        .reduce(num::integer::lcm)
        .unwrap()
}
