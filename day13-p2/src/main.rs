/*

*/


use regex::Regex;
use clap::{Arg, App};
use std:: fs;

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();

    let mut now = 100000000000494;
    let mut buses = lines[1].split(',').enumerate().filter(|x| x.1 != "x").map(|c| (c.0 as isize, c.1.parse::<isize>().unwrap())).collect::<Vec<(isize,isize)>>();
    buses.sort_by(|a,b | a.1.partial_cmp(&b.1).unwrap());
    let max = *buses.last().unwrap();
    buses = buses.iter().map(|x| ((x.0 - max.0).abs(), x.1)).collect();

    println!("{:?}", buses);
    while !buses.iter().map(|bus| -> bool {(now - bus.0) % bus.1 == 0 }).all(|x| x) {
        now += max.1;
    }

    println!("{}", now - max.0);
}
