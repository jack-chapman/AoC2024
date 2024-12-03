use regex::Regex;


fn main() {
    let input = include_str!("./input.txt");

    let result_1 = solve_1(input);
    let result_2 = solve_2(input);

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
}

fn solve_1(input: &str) -> u32 {
    let muls = parse(input);
    muls.into_iter().map(|(left, right)| left * right).sum()
}

fn solve_2(input: &str) -> u32 {
    let muls = parse_with_instruction(input);
    muls.into_iter().map(|(left, right)| left * right).sum()
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex
        .captures_iter(input)
        .map(|cap| cap.extract())
        .map(|(_, [left, right])| {
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

fn parse_with_instruction(input: &str) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let num = Regex::new(r"\d{1,3}").unwrap();
    let mut muls = vec![];
    let mut enabled = true;
    for pattern in regex.find_iter(input) {
        let pat = pattern.as_str();
        match pat {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if !enabled {
                    continue;
                } 
                let mut nums = num.find_iter(pat);
                let left = nums.next().unwrap().as_str();
                let right = nums.next().unwrap().as_str();
                muls.push((left.parse().unwrap(), right.parse().unwrap()))
            }
        }
    }

    muls
}
