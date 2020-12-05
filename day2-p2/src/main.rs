/*
--- Part Two ---
While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

1-3 a: abcde is valid: position 1 contains a and position 3 does not.
1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
How many passwords are valid according to the new interpretation of the policies?
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

    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();

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
