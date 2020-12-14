/*

*/


use regex::Regex;
use clap::{Arg, App};
use std::{collections::HashMap, fs, u64::MAX};

type Mask = Vec<char>;

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let mask_reg = Regex::new(r"^mask = ([\dX]+)$").unwrap();
    let mem_reg = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut mem = HashMap::<u64, u64>::new();
    let mut mask = Mask::new();

    for line in lines {
        if mask_reg.is_match(line) {
            let cap = &mask_reg.captures(line).unwrap()[1];
            mask = cap.chars().into_iter().collect();
        } else if mem_reg.is_match(line) {
            let cap = mem_reg.captures(line).unwrap();
            let addr: u64 = cap[1].parse().unwrap();
            let value: u64 = cap[2].parse().unwrap();

            let masked = apply_mask(&mask, value);
            mem.insert(addr, masked);
        } else {
            panic!("Invalid line")
        }
    }

    println!("{}", mem.iter().map(|a| a.1).sum::<u64>());
}

fn apply_mask(mask: &Mask, value: u64) -> u64 {
    println!("{:b}\n", value);
    let mut set_mask = 0;
    let mut unset_mask = 0;
    let slice_mask = { let mut a = 1; for _ in 0..36 { a <<= 1; a |= 1}; a};

    for c in mask {
        set_mask <<= 1;
        unset_mask <<= 1;
        match *c {
            '1' => {
                set_mask |= 1;
            },
            '0' => {
                unset_mask |= 1;
            },
            'X' => {},
            _ => {}
        }
    }

    println!("S: {:b}\nU: {:b}",set_mask,unset_mask);

    let mut n_value = value;
    n_value |= set_mask;
    n_value &= !unset_mask;
    n_value &= slice_mask;

    println!("res: {} {:b}", n_value, n_value);
    n_value
}
