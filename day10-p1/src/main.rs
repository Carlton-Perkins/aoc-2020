/*

*/

use clap::{App, Arg};
use itertools::Itertools;
use std::{collections::HashSet, fs};

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

    let lines: Vec<i32> = input.split('\n').filter(|s| !s.is_empty()).map(|x| x.parse::<i32>().unwrap()).sorted().collect();
    let end = *lines.iter().max().unwrap();

    let mut current = 0;
    let mut one_jump_count = 0;
    let mut tripple_jump_count = 1;
    while current != end {
        if lines.contains(&(current + 1)) {
            current = current + 1;
            one_jump_count += 1;
        } else if lines.contains(&(current + 2)) {
            current = current + 2;
        } else if lines.contains(&(current + 3)) {
            current = current + 3;
            tripple_jump_count += 1;
        } else {panic!("No hops possible")}
    }

    println!("{} {} -> {}", one_jump_count, tripple_jump_count, one_jump_count * tripple_jump_count);
}
