use std::{fs, str::FromStr, i32, collections::HashMap};

#[derive(Debug)]
struct MinMaxNum {
    idx: i32,
    num: String,
}

fn main() {
    let file:Vec<_> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut total = 0;

    for line in file {

        let mut alph_nums:Vec<_> = Vec::new(); 
        let nums: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        for num in nums {
            let v: &Vec<_> = &line.match_indices(num).collect();
            for (i,j) in v.iter() {
                alph_nums.push((i.to_owned(), j.to_owned()));
            }
        }

        let mut min_str: MinMaxNum = MinMaxNum{idx: 100, num: String::from("")};
        let mut max_str: MinMaxNum = MinMaxNum{idx: 0, num: String::from("")};

        for (j,k) in alph_nums.iter() {
            let idx = i32::from(*j as i8);
            if idx <= min_str.idx {
                min_str.num.clear();
                min_str.num.push_str(k);
                min_str.idx = idx;
            }

            if idx >= max_str.idx {
                max_str.num.clear();
                max_str.num.push_str(k);
                max_str.idx = idx;
            }
        }


        let numerical_digits: &Vec<_> = &line
            .chars()
            .enumerate()
            .filter(|(_,n)| n.is_numeric())
            .collect();

        for (j,k) in numerical_digits.iter() {
            let idx = i32::from(*j as i8);
            if idx <= min_str.idx {
                min_str.num.clear();
                min_str.num.push(*k);
                min_str.idx = idx;
            }

            if idx >= max_str.idx {
                max_str.num.clear();
                max_str.num.push(*k);
                max_str.idx = idx;
            }
        }

        //println!("min: {:?}\nmax: {:?}\n", min_str, max_str);


        let mut final_num = String::new();
        match &min_str.num  as &str{
            "zero" => final_num.push('0'),
            "one" => final_num.push('1'),
            "two" => final_num.push('2'),
            "three" => final_num.push('3'),
            "four" => final_num.push('4'),
            "five" => final_num.push('5'),
            "six" => final_num.push('6'),
            "seven" => final_num.push('7'),
            "eight" => final_num.push('8'),
            "nine" => final_num.push('9'),
            _ => final_num.push_str(&min_str.num)
        }

        match &max_str.num  as &str{
            "zero" => final_num.push('0'),
            "one" => final_num.push('1'),
            "two" => final_num.push('2'),
            "three" => final_num.push('3'),
            "four" => final_num.push('4'),
            "five" => final_num.push('5'),
            "six" => final_num.push('6'),
            "seven" => final_num.push('7'),
            "eight" => final_num.push('8'),
            "nine" => final_num.push('9'),
            _ => final_num.push_str(&max_str.num)
        }
        
        total = i32::from_str(&final_num.to_string()).unwrap() + total;
    }
    println!("total: {}", total);
}
