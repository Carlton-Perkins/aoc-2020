/*
--- Part Two ---
Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.

Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:

Action N means to move the waypoint north by the given value.
Action S means to move the waypoint south by the given value.
Action E means to move the waypoint east by the given value.
Action W means to move the waypoint west by the given value.
Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
Action F means to move forward to the waypoint a number of times equal to the given value.
The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.

For example, using the same instructions as above:

F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.

Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
*/

use cgmath::prelude::*;
use cgmath::Deg;
use clap::{App, Arg};
use regex::Regex;
use std::fs;

struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let matches = App::new("day1")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let line_reg = Regex::new(r"(\D)(\d+)").unwrap();

    let mut ship_pos = Position { x: (0), y: (0) };
    let mut way_pos = Position { x: (10), y: (1) };
    for line in lines {
        assert!(line_reg.is_match(line));

        let cap = line_reg.captures(line).unwrap();
        let command: &str = &cap[1];
        let value = cap[2].parse::<i32>().unwrap();

        println!(
            "Command: {} \t Value: {} \t Pos: {} {} \t Way: {} {}",
            command, value, ship_pos.x, ship_pos.y, way_pos.x, way_pos.y
        );
        match command {
            "N" => way_pos.y += value,
            "S" => way_pos.y -= value,
            "E" => way_pos.x += value,
            "W" => way_pos.x -= value,
            "L" => {
                way_pos = Position {
                    x: (way_pos.x * Deg::cos(Deg(value as f32)) as i32
                        - way_pos.y * Deg::sin(Deg(value as f32)) as i32),
                    y: (way_pos.y * Deg::cos(Deg(value as f32)) as i32
                        + way_pos.x * Deg::sin(Deg(value as f32)) as i32),
                }
            }
            "R" => {
                way_pos = Position {
                    x: (way_pos.x * Deg::cos(Deg(-value as f32)) as i32
                        - way_pos.y * Deg::sin(Deg(-value as f32)) as i32),
                    y: (way_pos.y * Deg::cos(Deg(-value as f32)) as i32
                        + way_pos.x * Deg::sin(Deg(-value as f32)) as i32),
                }
            }
            "F" => {
                ship_pos = Position {
                    x: (ship_pos.x + (way_pos.x * value)),
                    y: (ship_pos.y + (way_pos.y * value)),
                }
            }
            _ => {
                panic!("Unknown command")
            }
        }
    }

    println!(
        "Distance from Start {} {} -> {}",
        ship_pos.x.abs(),
        ship_pos.y.abs(),
        ship_pos.x.abs() + ship_pos.y.abs()
    );
}
