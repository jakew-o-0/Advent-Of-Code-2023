use std::fs;

fn main() {
    let file: String = fs::read_to_string("./input.txt").unwrap();

    let nums: Vec<_> = file
        .split(&[':', '|', '\n'][..])
        .map(|x| x.to_string())
        .filter(|x| !x.contains("Card"))
        .collect();

    for i in (0..nums.len()).step_by(2) {
        let winning_nums = nums
            .iter()
            .chars()
            .filter(|x| nums[i].as_bytes().contains(x))
            .collect();
    }
}
