/*
--- Day 5: Binary Boarding ---
You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.

You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.

Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.

For example, consider just the first seven characters of FBFBBFFRLR:

Start by considering the whole range, rows 0 through 127.
F means to take the lower half, keeping rows 0 through 63.
B means to take the upper half, keeping rows 32 through 63.
F means to take the lower half, keeping rows 32 through 47.
B means to take the upper half, keeping rows 40 through 47.
B keeps rows 44 through 47.
F keeps rows 44 through 45.
The final F keeps the lower of the two, row 44.
The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.

For example, consider just the last 3 characters of FBFBBFFRLR:

Start by considering the whole range, columns 0 through 7.
R means to take the upper half, keeping columns 4 through 7.
L means to take the lower half, keeping columns 4 through 5.
The final R keeps the upper of the two, column 5.
So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.

Here are some other boarding passes:

BFFFBBFRRR: row 70, column 7, seat ID 567.
FFFBBBFRRR: row 14, column 7, seat ID 119.
BBFFBBFRLL: row 102, column 4, seat ID 820.
As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?
*/

use clap::{Arg, App};
use std::{fs, cmp::max};
use regex::Regex;


fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split(char::is_whitespace).filter(|s| !s.is_empty()).collect();
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
