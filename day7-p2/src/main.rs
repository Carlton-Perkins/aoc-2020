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
            println!("Contains: {} {}",contained[1].parse::<i32>().unwrap(),  contained[2].to_string());
            contents.push((contained[1].parse().unwrap(),contained[2].to_string()));
        }

        bag_index.insert(cap[1].to_string(), Bag{contents});
    }

    println!("Total: {}", bags_inside(&bag_index, &"shiny gold".to_string()));
}

fn bags_inside(index: &HashMap<String, Bag>, bag: &String) -> i32 {
    println!("Checking {}", bag);
    let mut inside = 0;
    for con in &index[bag].contents {
        let con_inside = bags_inside(&index, &con.1);
        inside += con.0 + (con.0 * con_inside);
        println!("ConInside {}", con_inside);
    }
    inside
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
