/*--- Day 1: Report Repair ---
--- Part Two ---
The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?
*/

use clap::{Arg, App};
use std::fs;
use itertools::Itertools;

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split(char::is_whitespace).collect();
    let mut list: Vec<i32> = vec![];
    for line in lines {
        // println!("'{}'", line);
        let val: i32 = line.parse().unwrap();
        list.push(val);
    }

    let comb: Vec<Vec<i32>> = list.into_iter().combinations(3).collect();
    let target = 2020;

    for i in comb {
        if i[0] + i[1] + i[2] == target {
            println!("{} {} {} -> {}", i[0], i[1], i[2], i[0] * i[1] * i[2]);
            break;
        }
    }
}
