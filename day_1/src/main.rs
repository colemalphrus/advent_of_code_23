use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//fn get_num(input: &str) -> Option<i32> {
//    let s = input.to_string();
//    let digits: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
//
//    if digits.is_empty() {
//        None
//    } else {
//        let first = digits.first().unwrap();
//        let last = digits.last().unwrap();
//        // println!("{}{}", first, last);
//        format!("{}{}", first, last).parse::<i32>().ok()
//    }
//}

fn text_num(s: &str) -> Option<i32> {
    let mut v: Option<i32> = None;
    let mut map = HashMap::new();

    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    for (key, value) in &map {
        match s.find(key) {
            Some(_) => {
                v = Some(*value as i32);
                println!("FOUND::{} {}", s, value);
                break;
            }
            _ => continue,
        }
    }

    v
}

fn get_num2(input: &str) -> Option<i32> {
    let s = input.to_string();
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;

    for (index, ch) in s.char_indices() {
        if ch.is_digit(10) {
            first = ch.to_string().parse::<i32>().ok();
            break;
        } else {
            let slice = &s[0..index + 1];
            //println!("{}", slice);
            match text_num(slice) {
                Some(n) => {
                    first = Some(n);
                    break;
                }
                None => continue,
            }
        }
    }

    for (index, ch) in s.char_indices().rev() {
        if ch.is_digit(10) {
            last = ch.to_string().parse::<i32>().ok();
            break;
        } else {
            let slice = &s[index..];
            //println!("{} {}", slice, index);
            match text_num(slice) {
                Some(n) => {
                    last = Some(n);
                    break;
                }
                None => continue,
            }
        }
    }

    //println!("{} {} {}", first.unwrap(), last.unwrap(), s);
    match (first, last) {
        (Some(f), Some(l)) => format!("{}{}", f, l).parse::<i32>().ok(),
        _ => None,
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
    let file_path = "src/data.txt";

    let lines = read_lines(file_path)?;

    let mut numbers = Vec::new();

    for line in lines {
        if let Ok(ip) = line {
            match get_num2(&ip) {
                Some(num) => numbers.push(num),
                None => println!("oops"),
            }
        }
    }

    let sum: i32 = numbers.iter().sum();
    println!("ANSWER::{}", sum);

    Ok(())
}
