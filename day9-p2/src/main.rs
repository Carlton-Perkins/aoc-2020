/*
--- Part Two ---
The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.

Again consider the above example:

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list might be much longer.)

To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.

What is the encryption weakness in your XMAS-encrypted list of numbers?
*/

use clap::{App, Arg};
use itertools::Itertools;
use std::fs;

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

    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let data: Vec<i64> = lines.iter().map(|x| x.parse::<i64>().unwrap()).collect();

    let preamble = 25;

    let mut flawed_number = 0;
    for index in preamble..data.len() {
        let comb: Vec<i64> = data[index - preamble..index]
            .iter()
            .combinations(2)
            .map(|v| v[0] + v[1])
            .collect();
        if !comb.contains(&data[index]) {
            println!("Flawed index: {}, {}", index, data[index]);
            flawed_number = data[index];
            break;
        }
    }

    let mut search_size = 2;
    loop {
        for search_index in 0..data.len() - search_size {
            if flawed_number == data[search_index..search_index + search_size].iter().sum() {
                let res = data[search_index..search_index + search_size - 1]
                    .iter()
                    .minmax();
                let min;
                let max;
                match res {
                    itertools::MinMaxResult::MinMax(a, b) => {
                        min = a;
                        max = b
                    }
                    itertools::MinMaxResult::OneElement(a) => {
                        min = a;
                        max = a
                    }
                    itertools::MinMaxResult::NoElements => {
                        min = &0;
                        max = &0
                    }
                }
                println!("FOUND: {} {} {}", min, max, min + max);
                return;
            }
        }
        search_size += 1;
    }
}
