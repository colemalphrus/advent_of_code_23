use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn get_num(input: &str) -> Option<i32> {
    let digits: Vec<char> = input.chars().filter(|c| c.is_digit(10)).collect();

    if digits.is_empty() {
        None
    } else {
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        format!("{}{}", first, last).parse::<i32>().ok()
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    // Specify the file path here. Change "path/to/file.txt" to your file's path
    let file_path = "src/data.txt";

    let lines = read_lines(file_path)?;

    let mut numbers = Vec::new();

    for line in lines {
        if let Ok(ip) = line {
          match get_num(&ip) {
            Some(num) => numbers.push(num),
              None => println!("oops")
          }
        }
    }

    let sum: i32 = numbers.iter().sum();
    println!("{}", sum);

    Ok(())
}

