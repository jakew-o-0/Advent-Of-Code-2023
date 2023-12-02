use std::{fs, collections::HashMap};

fn main() {
    let file: Vec<_> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let colours_max_values: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut total = 0;
    let mut game_counter = 0;

    for line in file {
        game_counter = game_counter + 1;

        let s:Vec<_> = line
            .split(&[';', ',', ':'][..])
            .map(|x| x.to_string())
            .collect();


        let mut power: i32 = 1;
        for (key, _value) in colours_max_values.iter() {
            let colour_total = get_colour(&s, key);
            power = power * colour_total;
        }

        total = total + power;
    }
    println!("total: {}", total);
}


fn get_colour(games: &Vec<String>, colour: &str) -> i32 {
    let mut colour_nums: Vec<_> = games.iter()
        .filter(|x| x.contains(colour))
        .map(|x| x
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect::<String>()
        )
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    colour_nums.sort();

    return colour_nums[colour_nums.len() - 1];
}

