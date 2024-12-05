use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");

    let result_1 = solve_1(input);

    println!("Part 1: {result_1}");
}

fn solve_1(input: &str) -> u32 {
    let (rules, instructions) = parse(input);
    let mut count = 0;

    'outer: for instruction in instructions {
        // loop backwards over page numbers
        for (i, page) in instruction.iter().enumerate().rev() {
            // find rules for page number in rules map
            if let Some(rule) = rules.get(page) {
                // if instruction contains number in wrong order, continue to next instruction
                let filtered: HashSet<u32> = instruction.iter().rev().skip(instruction.len() - i).copied().collect();
                let intersection = filtered.intersection(rule).count();
                if intersection > 0 {
                    continue 'outer;
                }
            }
        }
        // if not, find middle number and add it to count
        let middle = instruction[instruction.len() / 2];
        count += middle;
    }

    count
}

fn parse(input: &str) -> (Rules, Vec<Instructions>) {
    let (rules_list, instructions_list) = input.split_once("\n\n").unwrap();
    let mut rules: Rules = HashMap::new();
    for raw_rule in rules_list.lines() {
        let (first, second) = raw_rule.split_once('|').unwrap();
        let first = first.parse().unwrap();
        let second = second.parse().unwrap();
        rules.entry(first).and_modify(|list| {
            list.insert(second);
        }).or_insert(HashSet::from([second]));
    }
    let instructions: Vec<Instructions> = instructions_list.lines().map(|line| {
        line.split(',').map(|n| n.parse().unwrap()).collect()
    }).collect();
    (rules, instructions)
}

type Rules = HashMap<u32, HashSet<u32>>;

type Instructions = Vec<u32>;
