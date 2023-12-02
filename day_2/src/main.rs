use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Color {
  blue: i32,
  red: i32,
  green: i32,
}

fn game_counts(game: &str) -> Color {
  println!("{}", game);

  let c  = Color{blue: 1, red: 2 ,green: 3};
  c
}

fn process_line(s: String) {
    let parts = s.split(":").collect::<Vec<&str>>();
    let games = parts[1].split(";").collect::<Vec<&str>>();
    let game_num: i32 = parts[0].split(" ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    println!("{}", game_num);
    for game in games {
       game_counts(game); 
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
    for line in lines {
        if let Ok(l) = line {
            process_line(l)
        }
    }

    Ok(())
}
