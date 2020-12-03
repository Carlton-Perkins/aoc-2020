/*
--- Part Two ---
Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

Right 1, down 1.
Right 3, down 1. (This is the slope you already checked.)
Right 5, down 1.
Right 7, down 1.
Right 1, down 2.
In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on each of the listed slopes?
*/

use clap::{Arg, App};
use std::fs;


fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split('\n').collect();

    let mut map: Vec<Vec<bool>> = vec![];
    for line in lines {
        let mut row: Vec<bool> = vec![];
        for c in line.chars() {
            let which = match c {
                '.' => false,
                '#' => true,
                _ => panic!()
            };
            row.push(which);
        }
        map.push(row);
    }

    // println!("{:?}", map);
    let input_slopes = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];

    let mut mult:i64 = 1;
    for slope in input_slopes {
        mult *= count_trees(map.clone(), slope.0, slope.1);
    }

    println!("Trees {}", mult);
}

fn count_trees(map: Vec<Vec<bool>>, slopex: usize, slopey: usize) -> i64 {
    let mut x = 0;
    let mut trees = 0;
    for y in (0..map.len()).filter(|x| x % slopey == 0) {
        // println!("Y {}", y);
        if map[y][x % (map[y].len())] {
            // println!("{} {}", y+1, x % (map[y].len()));
            trees += 1;
        }
        x += slopex;
    }
    trees
}
