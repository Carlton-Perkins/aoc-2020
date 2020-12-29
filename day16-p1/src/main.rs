/*

*/

use clap::{App, Arg};
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
};

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

    let lines: Vec<&str> = input.split("\n\n").filter(|s| !s.is_empty()).collect();
    let key_reg = Regex::new(r"(\w+): (\d+)\-(\d+) or (\d+)\-(\d+)").unwrap();
    let ticket_reg = Regex::new(r"(\d+),?").unwrap();

    // There should only be 3 groups in the dataset
    assert!(lines.len() == 3);

    let key_lines = lines[0];
    let nearby_tickets = lines[2];

    assert!(key_reg.is_match(key_lines));
    let mut key_values = HashMap::new();

    // Deal with the keys
    for key in key_reg.captures_iter(key_lines) {
        // println!("{}", key[0].to_string());
        let key_label = key[1].to_string();
        let first_range = key[2].parse::<i32>().unwrap()..=key[3].parse::<i32>().unwrap();
        let second_range = key[4].parse::<i32>().unwrap()..=key[5].parse::<i32>().unwrap();

        for r in first_range {
            key_values.insert(r, key_label.clone());
        }
        for r in second_range {
            key_values.insert(r, key_label.clone());
        }
    }

    // Deal with my ticket

    // Deal with nearby tickets
    let nearby_tickets_split: Vec<&str> = nearby_tickets.split('\n').collect();
    let mut sum_invalid = 0;
    for nearby_ticket in nearby_tickets_split {
        // println!("{}", nearby_ticket);
        let values: Vec<i32> = nearby_ticket
            .split(',')
            .filter(|x| ticket_reg.is_match(x))
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        for value in values {
            if !key_values.contains_key(&value) {
                println!("{}", value);
                sum_invalid += value;
            }
        }
    }
    println!("Sum invalid: {}", sum_invalid);
}
