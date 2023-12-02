use std::{fs, str::FromStr, i32};

// Part One
/*
fn main() {
    let file:Vec<_> = fs::read_to_string("/home/jake/code/Advent-Of-Code-2023/day1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut final_digits: Vec<String> = Vec::new();

    for line in file {
        let digits: Vec<_> = line
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect();

        final_digits.push(format!(
            "{}{}", 
            digits[0], 
            digits[digits.len() - 1]
        ));
    }
    println!("{:?}", final_digits);

    let total: i32 = final_digits
        .iter()
        .map(|x| -> i32 {FromStr::from_str(x).unwrap()})
        .sum();

    println!("total: {}", total)
}
*/


fn main() {
    let file:Vec<_> = fs::read_to_string("/home/jake/code/Advent-Of-Code-2023/day1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut final_digits: Vec<String> = Vec::new();

    for line in file {

    }
    println!("{:?}", final_digits);
}

fn find_nums(&line: &str) {
    let mut num: String = String::new();

    for i in  0..line.len(){
        match line[0..i] {
            String::from("") => num = format!("{}{}", num, "0"),
            _ => (),
        };

    }
}
