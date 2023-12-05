use std::fs;

fn main() {
    let file: String = fs::read_to_string("./input.txt").unwrap();

    let nums: Vec<_> = file
        .split(&[':', '|', '\n'][..])
        .map(|x| x.to_string())
        .filter(|x| !x.contains("Card"))
        .collect();

    let mut total = 0;

    for i in (0..nums.len() - 1).step_by(2) {
        let check_nums: Vec<_> = nums[i]
            .split(' ')
            .collect();

        let to_be_checked: Vec<_> = nums[i+1]
            .split(' ')
            .collect();
        
        let winning_nums: Vec<_> = to_be_checked
            .iter()
            .filter(|x| check_nums.contains(*x))
            .filter(|x| !x.is_empty())
            .collect();

        if winning_nums.is_empty() {
            continue;
        }
        total = total + i32::pow(2, winning_nums.len() as u32 - 1);
    }
    println!("total: {}", total);
}
