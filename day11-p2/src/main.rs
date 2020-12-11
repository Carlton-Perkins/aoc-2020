/*

*/

use clap::{App, Arg};
use itertools::Itertools;
use std::{collections::HashSet, fs, ops::Add};

type GameState = Vec<Vec<char>>;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    y: i32,
    x: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
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

    let mut state: GameState = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    loop {
        let mut new_state: GameState = state.clone();
        for y in 0..state.len() {
            for x in 0..state[y].len() {
                let seat = state[y][x];
                match seat {
                    'L' => {
                        if adj_seats(
                            &Position {
                                y: y as i32,
                                x: x as i32,
                            },
                            &state,
                        ) == 0
                        {
                            new_state[y][x] = '#';
                        }
                    }
                    '#' => {
                        if adj_seats(
                            &Position {
                                y: y as i32,
                                x: x as i32,
                            },
                            &state,
                        ) >= 5
                        {
                            new_state[y][x] = 'L';
                        }
                    }
                    '.' => {
                        new_state[y][x] = '.';
                    }
                    _ => panic!("Unknown State"),
                }
            }
        }
        if new_state == state {
            break;
        } else {
            state = new_state;
        }
    }

    let mut acc = 0;
    for y in 0..state.len() {
        for x in 0..state[y].len() {
            if state[y][x] == '#' {
                acc += 1;
            };
        }
    }
    println!("{:?}", state);

    println!("Count of filled seats: {}", acc);
}

fn adj_seats(pos: &Position, state: &GameState) -> usize {
    let mut count = 0usize;
    let slopes = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for slope in slopes {
        if ray_until_seat(*pos, Position { y:slope.0, x:slope.1 }, state) {
            count += 1;
        }
    }

    count
}

fn is_in_bounds(pos: Position, gs: &GameState) -> bool {
    if pos.y < 0 || pos.x < 0 {return false};
    pos.y >= 0 && pos.x >= 0 && pos.y as usize <= (gs.len() - 1) && pos.x as usize <= (gs[0].len() - 1)
}

fn ray_until_seat(start: Position, slope: Position, gs: &GameState) -> bool {
    let mut pos = start;
    loop {
        if !is_in_bounds(pos + slope, gs) {
            return false;
        }
        pos = pos + slope;
        if gs[pos.y as usize][pos.x as usize] == '#' {
            return true;
        } else if gs[pos.y as usize][pos.x as usize] == 'L' {
            return false;
        }
    }
}
