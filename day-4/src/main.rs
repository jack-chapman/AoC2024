fn main() {
    let input = include_str!("./input.txt");

    let result_1 = solve_1(input);
    let result_2 = solve_2(input);

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
}

fn solve_1(input: &str) -> u32 {
    let grid = parse(input);
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, first) in row.iter().enumerate() {
            if *first == "X" {
                // look right
                if let Some(i) = grid[y].get((x+1)..=(x+3)) {
                    let maybe_mas = i.join("");
                    if maybe_mas == "MAS" {
                        count += 1;
                    }
                }
                // look left
                if x.checked_sub(3).is_some() {
                    if let Some(i) = grid[y].get((x-3)..=(x-1)) {
                        let maybe_sam = i.join("");
                        if maybe_sam == "SAM" {
                            count += 1;
                        }
                    }
                }
                // look up
                if y.checked_sub(3).is_some() {
                    let maybe_sam = grid[y-3][x].clone() + &grid[y-2][x] + &grid[y-1][x];
                    if maybe_sam == "SAM" {
                        count += 1;
                    }
                }
                // look down
                if grid.get(y + 3).is_some() {
                    let maybe_mas = grid[y+1][x].clone() + &grid[y+2][x] + &grid[y+3][x];
                    if maybe_mas == "MAS" {
                        count += 1;
                    }
                }
                // diagonal top left
                if y.checked_sub(3).is_some() && x.checked_sub(3).is_some() {
                    let maybe_mas = grid[y-1][x-1].clone() + &grid[y-2][x-2] + &grid[y-3][x-3];
                    if maybe_mas == "MAS" {
                        count += 1;
                    }
                } 
                // diagonal top right
                if y.checked_sub(3).is_some() && grid[y].get(x + 3).is_some() {
                    let maybe_mas = grid[y-1][x+1].clone() + &grid[y-2][x+2] + &grid[y-3][x+3];
                    if maybe_mas == "MAS" {
                        count += 1;
                    }
                } 
                // diagonal bottom left
                if grid.get(y + 3).is_some() && x.checked_sub(3).is_some() {
                    let maybe_mas = grid[y+1][x-1].clone() + &grid[y+2][x-2] + &grid[y+3][x-3];
                    if maybe_mas == "MAS" {
                        count += 1;
                    }
                } 
                // diagonal bottom right
                if grid.get(y + 3).is_some() && grid[y].get(x + 3).is_some() {
                    let maybe_mas = grid[y+1][x+1].clone() + &grid[y+2][x+2] + &grid[y+3][x+3];
                    if maybe_mas == "MAS" {
                        count += 1;
                    }
                } 
            }
        } 
    }
    count
}

fn solve_2(input: &str) -> u32 {
    let grid = parse(input);
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, first) in row.iter().enumerate() {
            if *first == "A" {
                let mut top_left = None;
                let mut top_right = None;
                let mut bottom_left = None;
                let mut bottom_right = None;
                // top left
                if y.checked_sub(1).is_some() && x.checked_sub(1).is_some() {
                    let found = grid[y-1][x-1].clone();
                    if found != "M" && found != "S" {
                        continue;
                    }
                    top_left = Some(found);
                } 
                if top_left.is_none() {
                    continue;
                }
                // top right
                if y.checked_sub(1).is_some() && grid[y].get(x + 1).is_some() {
                    let found = grid[y-1][x+1].clone();
                    if found != "M" && found != "S" {
                        continue;
                    }
                    top_right = Some(found);
                } 
                if top_right.is_none() {
                    continue;
                }
                // bottom left
                if grid.get(y + 1).is_some() && x.checked_sub(1).is_some() {
                    let found = grid[y+1][x-1].clone();
                    if found != "M" && found != "S" {
                        continue;
                    }
                    bottom_left = Some(found);
                } 
                if bottom_left.is_none() {
                    continue;
                }
                // bottom right
                if grid.get(y + 1).is_some() && grid[y].get(x + 1).is_some() {
                    let found = grid[y+1][x+1].clone();
                    if found != "M" && found != "S" {
                        continue;
                    }
                    bottom_right = Some(found);
                } 
                if bottom_right.is_none() {
                    continue;
                }
                if let (Some(tl), Some(tr), Some(bl), Some(br)) = (top_left, top_right, bottom_left, bottom_right) {
                    let tl2br = format!("{tl}A{br}");
                    let tr2bl = format!("{tr}A{bl}");
                    if (tl2br == "MAS" || tl2br == "SAM") && (tr2bl == "MAS" || tr2bl == "SAM") {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn parse(input: &str) -> Vec<Vec<String>> {
    input.lines().map(|line| line.chars().map(|c| c.to_string()).collect()).collect()
}

