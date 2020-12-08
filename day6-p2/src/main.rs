/*
--- Part Two ---
As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b
This list represents answers from five groups:

In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
In the second group, there is no question to which everyone answered "yes".
In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
In the fourth group, everyone answered yes to only 1 question, a.
In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
*/

use std::collections::HashSet;
use clap::{Arg, App};
use std:: fs;

fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut count = 0;
    for group in groups {
        let mut allset = HashSet::new();
        let mut set = false;
        println!("-");
        for person in group.split("\n") {
            println!("{}", person);
            let mut pset = HashSet::new();
            for c in person.chars(){
                pset.insert(c);
            }
            if !set {
                allset = pset;
                set = true;
                println!("Init allset");
            } else {
                allset = allset.intersection(&pset).map(|x| *x).collect();
                println!("Common {:?}", allset);
            }
        }
        count += allset.len();
        println!("{}", allset.len());
    }

    println!("{}", count);
}
