/*

*/

use clap::{App, Arg};
use itertools::Itertools;
use std::{collections::HashSet, fs};

type GameState = Vec<Vec<char>>;
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
                        if adj_seats(y, x, &state) == 0 {
                            new_state[y][x] = '#';
                        }
                    }
                    '#' => {
                        if adj_seats(y, x, &state) >= 4 {
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
            if state[y][x] == '#' {acc+=1;};
        }
    }

    println!("Count of filled seats: {}", acc);
}

fn adj_seats(y: usize, x: usize, state: &GameState) -> usize {
    let mut count = 0usize;

    if y != 0 && x != 0 && state[y - 1][x - 1] == '#' {
        count += 1;
    }
    if y != 0 && state[y - 1][x] == '#' {
        count += 1;
    }
    if y != 0 && x != state[y].len() -1 && state[y - 1][x + 1] == '#' {
        count += 1;
    }
    if x != 0 && state[y][x - 1] == '#' {
        count += 1;
    }
    if x != state[y].len() -1 && state[y][x + 1] == '#' {
        count += 1;
    }
    if y != state.len() - 1 && x != 0 && state[y + 1][x - 1] == '#' {
        count += 1;
    }
    if y != state.len() - 1 && state[y + 1][x] == '#' {
        count += 1;
    }
    if y != state.len() - 1 && x != state[y].len() - 1 && state[y + 1][x + 1] == '#' {
        count += 1;
    }
    count
}
