/*

*/

use clap::{App, Arg};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
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
    let key_reg = Regex::new(r"(.*): (\d+)\-(\d+) or (\d+)\-(\d+)").unwrap();
    let ticket_reg = Regex::new(r"(\d+),?").unwrap();

    // There should only be 3 groups in the dataset
    assert!(lines.len() == 3);

    let key_lines = lines[0];
    let my_ticket_lines = lines[1];
    let nearby_tickets = lines[2];

    assert!(key_reg.is_match(key_lines));
    let mut key_values = HashMap::new();

    // Deal with the keys
    for key in key_reg.captures_iter(key_lines) {
        let key_label = key[1].to_string();
        let first_range = key[2].parse::<i32>().unwrap()..=key[3].parse::<i32>().unwrap();
        let second_range = key[4].parse::<i32>().unwrap()..=key[5].parse::<i32>().unwrap();
        let mut valid_range = HashSet::new();

        for r in first_range {
            valid_range.insert(r);
        }
        for r in second_range {
            valid_range.insert(r);
        }

        key_values.insert(key_label, valid_range);
    }

    // Deal with my ticket
    let my_ticket_split: Vec<&str> = my_ticket_lines
        .split('\n')
        .filter(|x| ticket_reg.is_match(x))
        .collect();
    assert!(my_ticket_split.len() == 1);
    let my_ticket: Vec<i32> = my_ticket_lines
        .split(|c| c == ',' || c == '\n')
        .filter(|x| ticket_reg.is_match(x))
        .filter(|x| x.chars().into_iter().all(char::is_numeric))
        .map(|x| {
            println!("'{}'", x);
            x.parse::<i32>().unwrap()
        })
        .collect();

    // Deal with nearby tickets
    let nearby_tickets_split: Vec<&str> = nearby_tickets
        .split('\n')
        .filter(|x| ticket_reg.is_match(x))
        .collect();
    let mut valid_tickets = vec![];
    for nearby_ticket in nearby_tickets_split {
        let values: Vec<i32> = nearby_ticket
            .split(',')
            .filter(|x| ticket_reg.is_match(x))
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let valid = values
            .iter()
            .all(|value| key_values.iter().any(|key| key.1.contains(value)));
        if valid {
            valid_tickets.push(values);
        };
    }
    println!("Valid count: {}", valid_tickets.len());

    // init search with all keys being valid for all fields
    let mut combos = HashMap::new();
    for idx in 0..key_values.len() {
        combos.insert(idx, key_values.keys().collect::<HashSet<&String>>());
    }

    // remove invalid combos
    for valid_ticket in valid_tickets.iter() {
        for field_idx in 0..valid_ticket.len() {
            for key_value in key_values.iter() {
                if !key_value.1.contains(&valid_ticket[field_idx]) {
                    if combos.contains_key(&field_idx) {
                        combos.get_mut(&field_idx).unwrap().remove(key_value.0);
                    }
                }
            }
        }
    }
    // resolve
    let mut done = false;

    while !done {
        for c in combos.clone().iter_mut() {
            if c.1.len() == 1 {
                for rem in combos.clone().iter() {
                    if c.0 != rem.0 {
                        let mut s = rem.1.clone();
                        s.remove(c.1.iter().take(1).map(|x| *x).collect::<Vec<&String>>()[0]);
                        combos.insert(*rem.0, s);
                    }
                }
            }
        }

        done = combos.iter().all(|x| x.1.len() == 1);
    }
    println!("Combos {:?}", combos);

    println!(
        "'{:?}'",
        combos
            .iter()
            .filter(
                |x| x.1.iter().take(1).map(|x| *x).collect::<Vec<&String>>()[0]
                    .contains("departure")
            )
            .fold(1u64, |acc, x| acc * my_ticket[*x.0] as u64)
    );
}
