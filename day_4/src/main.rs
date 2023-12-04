use std::collections::hash_map::Entry;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{cmp::min, collections::HashMap, fs::read_to_string};

fn process_hand(game: &str) -> Option<i32> {
    let game = game.split(":").collect::<Vec<&str>>()[1]
        .split("|")
        .collect::<Vec<&str>>();
    let win_nums = game[0].trim().split(" ").collect::<Vec<&str>>();
    let my_nums = game[1].trim().split(" ").collect::<Vec<&str>>();
    let mut win_set: HashSet<&str> = win_nums.clone().into_iter().collect();
    win_set.remove("");
    let mut out: i32 = 0;
    for i in my_nums {
        match win_set.contains(i) {
            true if out == 0 => out = 1,
            true => out = out * 2,
            false => continue,
        }
    }

    Some(out)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let file_path = "src/data.txt";

    let lines = read_lines(file_path)?;

    let mut numbers = Vec::new();

    for line in lines {
        if let Ok(ip) = line {
            match process_hand(&ip) {
                Some(num) => numbers.push(num),
                None => println!("oops"),
            }
        }
    }

    let sum: i32 = numbers.iter().sum();
    println!("ANSWER::{}", sum);

    Ok(())
}
