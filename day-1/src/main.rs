fn main() {
    let input = include_str!("./input.txt");

    let result = solve(input);
    let result_2 = solve_2(input);

    println!("{result}");
    println!("{result_2}");
}

fn solve(input: &str) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for (l, r) in input.lines().map(|line| line.split_once("   ").unwrap()) {
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }
    left.sort();
    right.sort();
    left.iter().zip(right.iter()).map(|(l, r)| {
        u32::abs_diff(*l, *r)
    }).sum()
}

fn solve_2(input: &str) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for (l, r) in input.lines().map(|line| line.split_once("   ").unwrap()) {
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }
    left.into_iter().map(|num| {
        num * count_occurances(&right, num)
    }).sum()
}

fn count_occurances(list: &[u32], num: u32) -> u32 {
    list.iter().filter(|i| **i == num).count().try_into().unwrap()
}
