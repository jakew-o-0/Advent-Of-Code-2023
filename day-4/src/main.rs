use std::{fs, collections::HashMap, i128, i32};

fn main() {
    let file: String = fs::read_to_string("./dummy-test.txt").unwrap();

    let nums: Vec<_> = file
        .split(&[':', '|', '\n'][..])
        .map(|x| x.to_string())
        .filter(|x| !x.contains("Card"))
        .collect();

    let mut found_nums:HashMap<usize, i32> = HashMap::new();
    

    let mut idx = 1;
    for i in (0..nums.len() -1).step_by(2) {
        
            found_nums
                .entry(idx)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        println!("before: {:?}", found_nums);

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

        println!("nums: {:?}", winning_nums);


        for j in 0..winning_nums.len() {
            println!("i: {}", idx+j+1);
            let mut incr = 0;
            if found_nums[&idx] > 1{
                incr = idx as i32;
            } else {
                incr = 1;
            }
            found_nums
                .entry(j+idx+1)
                .and_modify(|x| *x += incr)
                .or_insert(1);
        }
        
        idx += 1;
        println!("after: {:?}\n", found_nums);
    }

    let mut total = 0;
    for (k,v) in found_nums.iter() {
        total += v;
    }


    println!("{:?}", total);
}

