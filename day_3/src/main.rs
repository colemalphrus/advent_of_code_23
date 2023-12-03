use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn create_special_chars_set() -> HashSet<char> {
    let special_chars: Vec<char> = vec![
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '/', ':', ';', '<', '=',
        '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ];
    special_chars.into_iter().collect()
}

fn adj_char(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let special_chars_set = create_special_chars_set();
    let row: Option<&Vec<char>> = grid.get(y);
    let c: Option<&char> = match row {
        Some(r) => r.get(x),
        None => None,
    };

    match c {
        Some(e) => special_chars_set.contains(e),
        None => false,
    }
}

fn is_part_number(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            let neighbor_x = x as isize - 1 + i as isize;
            let neighbor_y = y as isize - 1 + j as isize;

            if i == 1 && j == 1 {
                continue;
            }

            match adj_char(grid, neighbor_x as usize, neighbor_y as usize) {
                true => return true,
                false => continue,
            }
        }
    }

    false
}

fn char_2_num(parts: &Vec<char>) -> Result<i32, std::num::ParseIntError> {
    let pn: String = parts.into_iter().collect();
    let out = pn.parse();
    out
}

fn part_1(grid: Vec<Vec<char>>) -> Vec<i32> {
    let mut part_numbers: Vec<i32> = Vec::new();
    let mut part_num_vec: Vec<char> = Vec::new();
    let mut is_part = false;

    for (yi, row) in grid.iter().enumerate() {
        for (xi, c) in row.iter().enumerate() {
            // println!("y_axis::{} x_axis::{} value::{}",yi, xi, c);
            // adj_char(&grid, xi, yi);
            if c.is_digit(10) {
                part_num_vec.push(*c);
                if is_part_number(&grid, xi, yi) {is_part = true;};
            } else {
                if !is_part {
                    part_num_vec = Vec::new();
                    continue;
                }
                match char_2_num(&part_num_vec) {
                    Ok(i) => {
                        part_numbers.push(i);
                        part_num_vec = Vec::new();
                        is_part = false;
                    }
                    _ => continue,
                }
            }
        }
    }

    part_numbers
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
    let mut out: i32 = 0;
    let lines = read_lines(file_path)?;
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if let Ok(l) = line {
            let v = l.chars().collect::<Vec<char>>();
            grid.push(v)
        }
    }
    let mut part_1_answer = 0;
    let parts = part_1(grid);
    for i in parts {
        part_1_answer += i;
        
        println!("PART::{} ANSWER::{}", i, part_1_answer);
    }
    Ok(())
}
