/*

*/

use clap::{Arg, App};
use std:: fs;

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let mut lines: Vec<usize> = input.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let target = 2020;

    while lines.len() != target {
        let current = lines.len();
        if !lines.contains(lines.last().unwrap()) {
            lines.push(0);
        } else {
            let last = lines.iter().rposition(|x| x == lines.last().unwrap()).unwrap();
            let first = lines.iter().position(|x| x == lines.last().unwrap()).unwrap();
            println!("C:{} F:{} L:{} Val:{}", current, first, last, &lines[current - 1]);
            if first == current - 1 {
                lines.push(0);
            } else {
                let second_last = lines[0..last].iter().rposition(|x| x == lines.last().unwrap()).unwrap();
                lines.push(last - second_last);
            }
        }
    }

    println!("{:?}", lines);
    println!("{}", lines.last().unwrap());
}
