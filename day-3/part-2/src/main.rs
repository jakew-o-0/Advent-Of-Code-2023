use std::fs;


fn main() {
    let file:String = fs::read_to_string("../input.txt").unwrap();
    let grid: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut total = 0;
    for col in 0..grid.len() {
        for row in 0..grid[col].len() {
            if grid[col][row] == '*' {
                let num = find_adjacent(&grid, col, row);
                total = total + num;
            }
        }
    }

    println!("total {:?}", total);
}

fn find_adjacent(grid: &Vec<Vec<char>>, col: usize, row: usize) -> i32 {
    let directions: [(isize, isize); 8] = [(1, 0), (1,1), (1,-1), (0,1), (0,-1), (-1,0), (-1,-1), (-1,1)];
    let mut nums: Vec<i32> = Vec::new();

    for (dy, dx) in directions {
        if (col as isize)+dy >=0 &&
        (col as isize)+dy < grid.len() as isize &&
        row as isize +dx >= 0 &&
        row as isize +dx < grid[col].len() as isize {
            let (new_row, new_col) = (row as isize + dx, col as isize + dy);

            let new_num = extract_num(grid, new_col as usize, new_row as usize);
            if !nums.iter().any(|n| *n == new_num) {
                nums.push(new_num);
            }
        }
    }

    let mut total = 1;
    for n in nums {
        total = total * n;
    }
    return total;
}

fn extract_num(grid: &Vec<Vec<char>>, col: usize, mut row: usize) -> i32 {
    while row as isize - 1 >= 0 && grid[col][row].is_numeric() {
        row = row -1;
    }

    let mut num = 0;
    while grid[col][row].is_numeric() {
        num = num*10 + (grid[col][row].to_digit(10).unwrap());
        row = row + 1;
    }
    return num as i32;
}
