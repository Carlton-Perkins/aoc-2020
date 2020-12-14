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

    let now = lines[0].parse::<i64>().unwrap();
    let buses = lines[1].split(',').filter(|x| *x != "x").map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut time_til: Vec<(i64,i64)> = buses.iter().map(|bus| (*bus, bus - (now % bus))).collect();
    time_til.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

    println!("{:?} {}",time_til , time_til[0].0 * time_til[0].1);
}
