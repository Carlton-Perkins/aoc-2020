/*

*/

use clap::{Arg, App};
use std::{collections::HashMap, fs};

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<usize> = input.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    let start = lines.len();
    let mut last = *lines.last().unwrap();

    for each in 0..lines.len() {
        map.insert(lines[each], (each,0));
    }

    let target = 10 -1;

    for index in start..=target {
        if !map.contains_key(&last) {
            map.insert(last,(index,0));
            last = 0;
        } else {
            let val = map.get(&last).unwrap();
            let second = val.0;
            let first = index;
            map.insert(last, (first,second));
            last = first - second;
        }
    }

    println!("{:?}", map);
    println!("{}", last);
}
