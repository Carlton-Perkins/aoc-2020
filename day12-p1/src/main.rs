/*
--- Day 12: Rain Risk ---
Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!

Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.

The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:

Action N means to move north by the given value.
Action S means to move south by the given value.
Action E means to move east by the given value.
Action W means to move west by the given value.
Action L means to turn left the given number of degrees.
Action R means to turn right the given number of degrees.
Action F means to move forward by the given value in the direction the ship is currently facing.
The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)

For example:

F10
N3
F7
R90
F11
These instructions would be handled as follows:

F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
N3 would move the ship 3 units north to east 10, north 3.
F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
F11 would move the ship 11 units south to east 17, south 8.
At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?
*/


use regex::Regex;
use clap::{Arg, App};
use std:: fs;

struct Position {
    x: i32,
    y: i32,
}

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let line_reg = Regex::new(r"(\D)(\d+)").unwrap();

    let mut pos = Position{x: (0), y: (0)};
    let mut rot: i32 = 1;
    for line in lines {
        assert!(line_reg.is_match(line));

        let cap = line_reg.captures(line).unwrap();
        let command: &str = &cap[1];
        let value = cap[2].parse::<i32>().unwrap();

        println!("Command: {} \t Value: {} \t Pos: {} {} \t Rot: {}", command, value, pos.x, pos.y, rot);
        match command {
            "N" => { pos.y += value},
            "S" => { pos.y -= value},
            "E" => { pos.x += value},
            "W" => { pos.x -= value},
            "L" => { rot = (rot - (value / 90)) % 4; if rot < 0 {rot = 4 + rot}},
            "R" => { rot = (rot + (value / 90)) % 4},
            "F" => {
                match rot {
                    0 => {pos.y += value},
                    1 => {pos.x += value},
                    2 => {pos.y -= value},
                    3 => {pos.x -= value},
                    _ => {panic!("Invalid rot")}
                }
            },
            _ => {panic!("Unknown command")},
        }
    }

    println!("Distance from Start {} {} -> {}", pos.x.abs(), pos.y.abs(), pos.x.abs() + pos.y.abs());
}