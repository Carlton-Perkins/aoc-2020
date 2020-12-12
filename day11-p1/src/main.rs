/*
--- Day 11: Seating System ---
Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).

The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
Otherwise, the seat's state does not change.
Floor (.) never changes; seats don't move, and nobody sits on the floor.

After one round of these rules, every seat in the example layout becomes occupied:

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
After a second round, the seats with four or more occupied adjacent seats become empty again:

#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
This process continues for three more rounds:

#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?
*/

use clap::{App, Arg};

use std::fs;

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
