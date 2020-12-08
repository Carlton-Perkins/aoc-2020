/*

*/

use std::collections::{HashMap, HashSet};
use clap::{Arg, App};
use std:: fs;
use regex::Regex;
use itertools::Itertools;

#[derive(Clone)]
struct Bag {
    contents: Vec<(i32, String)>,
}

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let line_reg = Regex::new(r"^(\w+ \w+) bags contain (.*.)$").unwrap();
    let num_bag_reg = Regex::new(r"(\d+) (\w+ \w+) bag[s]?").unwrap();

    let mut bag_index = HashMap::new();
    for line in lines {
        println!("{}", line);
        assert!(line_reg.is_match(line));

        let cap = line_reg.captures(line).unwrap();
        let mut contents = vec![];

        for contained in num_bag_reg.captures_iter(&cap[2]) {
            println!("Contains: {}", contained[0].to_string());
            contents.push((contained[1].parse().unwrap(),contained[2].to_string()));
        }

        bag_index.insert(cap[1].to_string(), Bag{contents});
    }

    let mut targets = vec!["shiny gold".to_string()];
    let mut found = vec![];
    while !targets.is_empty() {
        let target = targets.pop().unwrap();
        println!("Finding {}", target);

        for bag in bag_index.keys() {
            if *bag == target || targets.contains(bag) || found.contains(bag){
                continue;
            } else {
                if can_contain(&bag_index, bag, &target) {
                    targets.push(bag.clone());
                }
            }
        }
        found.push(target.clone());
    }

    println!("Total: {}", found.len());
}

fn can_contain(index: &HashMap<String, Bag>, bag: &String, target: &String) -> bool {
    for con in &index[bag].contents {
        if &con.1 == target {
            return true;
        }
    }
    false
}

// fn can_bag_hold(map: &HashMap<String, Bag>, bag: &String, target: &String, checked: &Vec<String>) -> bool {
//     // println!("Checking {}", bag);
//     if map[bag].contents.is_empty() {
//         return false;
//     } else {
//         let mut lchecked = checked.clone();
//         for cbag in &map[bag].contents {
//             // println!("--Contains {}", cbag.1);
//             if lchecked.contains(&cbag.1) {
//                 break;
//             } else {
//                 lchecked.push(cbag.1.clone());
//             }

//             if cbag.1 == *target {
//                 return true;
//             } else {
//                 if can_bag_hold(map, &cbag.1, target, &lchecked) {
//                     return true;
//                 }
//             }
//         }
//     }
//     false
// }
