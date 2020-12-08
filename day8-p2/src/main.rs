/*
--- Part Two ---
After some careful analysis, you believe that exactly one instruction is corrupted.

Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)

The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.

For example, consider the same program from above:

nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp instructions, the program will still eventually find another jmp instruction and loop forever.

However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:

nop +0  | 1
acc +1  | 2
jmp +4  | 3
acc +3  |
jmp -3  |
acc -99 |
acc +1  | 4
nop -4  | 5
acc +6  | 6
After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).

Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?
*/

use clap::{Arg, App};
use std:: fs;
use regex::Regex;


fn main() {

    let matches = App::new("day1").arg(
        Arg::with_name("file").short("f").long("file").takes_value(true)
    ).get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let line_reg = Regex::new(r"(\w+) ([\+\-]\d+)").unwrap();

    let mut lines_mutated = vec![];
    loop {
        println!("Mutated size {:?}", lines_mutated);
        let mut lines_visited = vec![];

        let mut acc = 0;
        let mut pc:i32 = 0;
        let mut has_mutated = false;
        loop {
            let line = lines[pc as usize];
            assert!(line_reg.is_match(line));
            lines_visited.push(pc);
            let cap = line_reg.captures(line).unwrap();
            let op = &cap[1];
            let val = &cap[2];
            let mutate = !lines_mutated.contains(&pc) && !has_mutated;

            let mut_op = match op {
                "nop" => {
                    // if mutate {println!("Mutated {}", pc)};
                    if mutate {
                        lines_mutated.push(pc);
                        has_mutated = true;
                    }
                    if mutate {"jmp"} else {op}},
                "jmp" => {
                    // if mutate {println!("Mutated {}", pc)};
                    if mutate {
                        lines_mutated.push(pc);
                        has_mutated = true;
                    };
                    if mutate {"nop"} else {op}
                },
                _ => op,
            };
            match mut_op {
                "nop" => {
                    pc += 1;
                },
                "acc" => {
                    acc += val.parse::<i32>().unwrap();
                    pc += 1;
                },
                "jmp" => {
                    pc += val.parse::<i32>().unwrap();
                },
                _ => panic!("Unknown instruction {}", op),
            }

            if lines_visited.contains(&pc) {
                println!("Stopping due to loop");
                break;
            }
            if pc as usize == lines.len() {
                println!("ACC: {}", acc);
                return;
            }
        }
    }
}
