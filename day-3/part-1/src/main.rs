use std::fs;

fn main() { 
    let file: Vec<_> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    // iterate through
    // if a num -> push to cur_num
    // check if valid 
    // if not a num check if all nums are valid -> add/or not

    let mut total = 0;
    for (i, line) in file.iter().enumerate() {
        let mut cur_num: String = String::new();
        let mut is_valid: String = String::new();

        for (j, ch) in line.chars().enumerate() {

            if ch.is_ascii_digit() {
                cur_num.push(ch);
                is_valid.push_str(&validate(i, j, &file));
            }
            else {
                if is_valid.contains("true") {
                    total = total + cur_num.parse::<i32>().unwrap();  
                    println!("accepted num: {}", cur_num);
                }
                cur_num.clear();
                is_valid.clear();
            }
            if j == line.len() - 1 {
                if is_valid.contains("true") {
                    total = total + cur_num.parse::<i32>().unwrap();  
                    println!("accepted num: {}", cur_num);
                }
            }
        }
    }
    println!("total {}", total);
}

fn validate(i: usize, j: usize, file: &Vec<String>) -> String {
    if j < file[i].len() - 1 {
        let char = file[i].as_bytes()[j+1];
        if char.is_ascii_punctuation() && char != b'.' {
            return "true".to_string();
        }
    } 

    if j > 0 {
        let char = file[i].as_bytes()[j-1];
        if char.is_ascii_punctuation() && char != b'.' {
            return "true".to_string();
        }
    }

    if i < file.len() - 1 {
        let char = file[i+1].as_bytes()[j];
        if  char.is_ascii_punctuation() && char != b'.' {
            return "true".to_string();
        }
    }

    if i > 0 {
        let char = file[i-1].as_bytes()[j];
        if  char.is_ascii_punctuation() && char != b'.' {
            return "true".to_string();
        }
    }

    if i < file.len() - 1 && j < file[i].len() - 1 {
        let char = file[i+1].as_bytes()[j+1];
        if  char.is_ascii_punctuation() && char != b'.' {
            return "true".to_string();
        }
    }

    if i > 0 && j > 0{
        let char = file[i-1].as_bytes()[j-1];
        if  char.is_ascii_punctuation() && char != b'.' {
            return "true".to_string();
        }
    }

    if i < file.len() - 1 && j > 0{
        let char = file[i+1].as_bytes()[j-1];
        if  char.is_ascii_punctuation() && char != b'.' {
            return "true".to_string();
        }
    }

    if i > 0 && j < file[i].len() - 1 {
        let char = file[i-1].as_bytes()[j+1];
        if  char.is_ascii_punctuation() && char != b'.' {
            return "true".to_string();
        }
    }

    return "false".to_string();
}
