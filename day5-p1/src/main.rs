use clap::{Arg, App};
use std::{fs, cmp::max};
use regex::Regex;


fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split(char::is_whitespace).collect();
    let line_reg = Regex::new(r"([FB]{7})([LR]{3})").unwrap();

    let mut max_id = 0;

    for line in lines {
        assert!(line_reg.is_match(line));

        let cap = line_reg.captures(line).unwrap();
        let fb = &cap[1];
        let lr = &cap[2];

        println!("{}\t{}", fb, lr);
        let row;
        let col;
    {
        let mut min = 0usize;
        let mut max = 127usize;
        let mut inc = 64usize;
        for c in fb.chars() {
            print!("{} {} {} ", c, min, max);
            println!("{}", inc);
            match c {
                'B' => min = min + inc,
                'F' => max = max - inc,
                _ => panic!("Invalid FB"),
            }
            inc /= 2;
        }
        println!("\t{} {}", min, max);
        assert!(min == max);
        row = min;
    }
    {
        let mut min = 0usize;
        let mut max = 7usize;
        let mut inc = 4usize;
        for c in lr.chars() {
            print!("{} {} {} ", c, min, max);
            println!("{}", inc);
            match c {
                'R' => min = min + inc,
                'L' => max = max - inc,
                _ => panic!("Invalid FB"),
            }
            inc /= 2;
        }
        println!("\t{} {}", min, max);
        assert!(min == max);
        col = min;
    }
        let id = (row * 8) + col;
        println!("ID: {}", id);

        max_id = max(id, max_id);
    }

    println!("MaxId: {} ", max_id);
}
