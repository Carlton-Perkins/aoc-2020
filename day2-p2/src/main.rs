use regex::Regex;
use clap::{Arg, App};
use std::fs;
use itertools::Itertools;

struct Password {
    min: u8,
    max: u8,
    char: char,
    passwd: String
}

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split('\n').collect();

    let parse_reg = Regex::new(r"(\d+)\-(\d+) (.): (.+)").unwrap();

    let mut count_valid = 0;
    for line in lines {
        // println!("{}", line);
        assert!(parse_reg.is_match(line));
        let cap = parse_reg.captures(line).unwrap();

        let p = Password{min: cap[1].parse().unwrap(), max: cap[2].parse().unwrap(), char: cap[3].parse().unwrap(), passwd: cap[4].parse().unwrap()};
        if validate_password(p){
            count_valid = count_valid + 1;
            println!("Valid: {}", line)
        }
    }
    println!("Correct Count: {}", count_valid);
}


fn validate_password(p: Password) -> bool {
    // let mut count = 0;
    let mut positions = 0;
    for each in p.passwd.chars().enumerate() {
        if each.1 == p.char {
            // count = count + 1;

            if each.0 + 1 == p.min.into() || each.0 + 1 == p.max.into() {
                positions = positions + 1;
            }
        }
    }

    positions == 1
}
