/*
--- Part Two ---
Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

What is the ID of your seat?
*/

use clap::{Arg, App};
use std::{collections::HashSet, cmp::max, fs};
use regex::Regex;
use itertools::Itertools;


fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split(char::is_whitespace).filter(|s| !s.is_empty()).collect();
    let line_reg = Regex::new(r"([FB]{7})([LR]{3})").unwrap();

    let mut max_id = 0;
    let mut ids = HashSet::new();

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
        ids.insert(id);
    }

    println!("MaxId: {} ", max_id);

    for (a,b) in ids.into_iter().sorted().tuple_windows() {
        if a != b - 1 {
            println!("Your ID: {}", a + 1);
        }
    }
}
