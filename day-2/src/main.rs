fn main() {
    let input = include_str!("./input.txt");

    let result_1 = solve_1(input);
    let result_2 = solve_2(input);

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
}

fn solve_1(input: &str) -> u32 {
    input.lines().map(|report| {
        let values: Vec<u32> = report.split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect();
        test_values(&values)
    }).filter(|v| *v).count() as u32
}

fn solve_2(input: &str) -> u32 {
    input.lines().map(|report| {
        let values: Vec<u32> = report.split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect();
        let mut ok = test_values(&values);
        if ok {
            return true;
        }
        let mut removed_index = 0;
        while !ok {
            if removed_index >= values.len() {
                return false;
            }
            let mut temp_vec = values.clone();
            temp_vec.remove(removed_index);
            removed_index += 1;
            ok = test_values(&temp_vec);
        };
        ok
    }).filter(|v| *v).count() as u32
}

fn small_enough_step(a: u32, b: u32) -> bool {
    matches!(a.abs_diff(b), 1..=3)
}

fn test_values(values: &[u32]) -> bool {
    let mut prev = values[0];
    let inc = values[0] < values[1];
    for val in values.iter().skip(1) {
        if (inc && *val > prev) || (!inc && *val < prev) {
            if !small_enough_step(*val, prev) {
                return false;
            }
        } else {
            return false;
        }
        prev = *val;
    }
    true
}
