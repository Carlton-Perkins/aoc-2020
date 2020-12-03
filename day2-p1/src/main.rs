/*
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?

Your puzzle answer was 569.


*/


use regex::Regex;
use clap::{Arg, App};
use std::fs;

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
        }
    }
    println!("Correct Count: {}", count_valid);
}


fn validate_password(p: Password) -> bool {
    let mut count = 0;
    for each in p.passwd.chars() {
        if each == p.char {
            count = count + 1;
        }
    }

    count <= p.max && count >= p.min
}
