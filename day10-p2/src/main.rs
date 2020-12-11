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

    let mut count  = 1;
    for current in lines.clone() {
        let mut icount = 1;
        if lines.contains(&(current + 1)) {
            icount += 1;
        }
        if lines.contains(&(current + 2)) {
            // icount += 1;
        }
        if lines.contains(&(current + 3)) {
            icount += 1;
        }
        count *= icount

    }

    println!("{}",count);
}
