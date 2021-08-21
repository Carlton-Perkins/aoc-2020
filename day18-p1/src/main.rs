/*

*/


use regex::Regex;
use clap::{App, Arg};
use std::fs;
use itertools::Itertools;
use crate::Expr::{LeftParen, Hole};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Expr {
    Number(i64),
    Plus,
    Mult,
    LeftParen,
    RightParen,
    Hole(Box<Vec<Expr>>)
}

fn into_raw_expr(string: String) -> Vec<Expr> {
    return string
        .replace("(", "( ")
        .replace(")", " )")
        .split(' ')
        .map(|s| match s {
        "(" => Expr::LeftParen,
        ")" => Expr::RightParen,
        "+" => Expr::Plus,
        "*" => Expr::Mult,
        n if n.parse::<i64>().is_ok() => Expr::Number(n.parse::<i64>().unwrap()),
        _ => panic!("Unknown symbol {}", s)
    }).collect()
}

fn replace_paren_pair_with_hole(data: Vec<Expr>) -> Vec<Expr> {
    let mut iter = data.into_iter();

    let mut acc = Vec::new();
    let start = loop {
        if let Some(e) = iter.next() {
            if e == LeftParen {
                break acc;
            }
            else {
                acc.push(e);
                continue;
            }
        } else {
            break acc;
        }
    };

    let mut acc = Vec::new();
    let mut paren_count = 1;
    let new_hole_body = loop {
        if let Some(e) = iter.next() {
            match e {
                Expr::LeftParen => { paren_count += 1 }
                Expr::RightParen => { paren_count -= 1 }
                _ => {
                }
            }
            if paren_count == 0 {
                break acc
            } else {
                acc.push(e);
                continue
            }
        } else {
            break Vec::new()
        }
    };

    let rest = iter.collect_vec();

    // println!("{:?} | {:?} | {:?}", start, new_hole_body, rest);

    return if new_hole_body.len() > 0 {
        [start,
            vec![
                Hole(
                    Box::new(
                        replace_paren_pair_with_hole(
                            new_hole_body
                        )
                    )
                )
            ],
            replace_paren_pair_with_hole(rest)]
            .concat()
    } else {
        start
    }
}

fn expr_eval(data: Vec<Expr>) -> i64 {
    use Expr::*;
    let mut buf = data.iter().take(3).collect_vec();
    if let [a, op, b] =  buf[..] {
        // println!("Eval-ing {:?}, {:?}, {:?}", a, op, b);
        let a_val = match a {
            Number(val) => *val,
            Hole(vbox) => expr_eval(vbox.to_vec()),
            _ => panic!("Unsupported A value {:?}", a)
        };

        let b_val = match b {
            Number(val) => *val,
            Hole(vbox) => expr_eval(vbox.to_vec()),
            _ => panic!("Unsupported B value {:?}", b)
        };

        return match op {
            Mult => expr_eval(vec![vec![Number(a_val*b_val)], data.into_iter().skip(3).collect_vec()].concat()),
            Plus=> expr_eval(vec![vec![Number(a_val+b_val)], data.into_iter().skip(3).collect_vec()].concat()),
            _ => panic!("Unsupported operator {:?}", op)
        }

    } else {
        // println!("No work to do...");
        return match data.iter().next().unwrap() {
            Number(n) => *n,
            _ => {panic!("Nothing in the acc slot")}
        };
    }
}

fn main() {
    let matches = App::new("day18")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<String> = input.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    println!("{}", lines.iter().map(|x| expr_eval(
        replace_paren_pair_with_hole(
            into_raw_expr(
                x.into())))).sum::<i64>());
}


#[cfg(test)]
mod test_expr {
    use super::*;
    use Expr::*;

    #[test]
    fn test_raw_expr() {
        let tests = vec![
            ("1 + 2 * 3", vec![Number(1), Plus, Number(2), Mult, Number(3)]),
            ("2 * 3 + (4 * 5)", vec![Number(2), Mult, Number(3), Plus, LeftParen, Number(4), Mult, Number(5), RightParen]),
            // ("5 + (8 * 3 + 9 + 3 * 4 * 3)", ),
            // ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", ),
            // ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", ),
        ];

        for (input,expected_out) in tests {
            assert_eq!(into_raw_expr(input.into()), expected_out);
        }
    }

    #[test]
    fn test_into_hole() {
        let tests = vec![
            ("1 + 2 * 3", vec![Number(1), Plus, Number(2), Mult, Number(3)]),
            ("2 * 3 + (4 * 5)", vec![Number(2), Mult, Number(3), Plus, Hole(Box::new(vec![Number(4), Mult, Number(5)]))]),
            // ("5 + (8 * 3 + 9 + 3 * 4 * 3)", ),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
                vec![
                    Number(5), Mult, Number(9), Mult, Hole(Box::new(vec![
                        Number(7), Mult, Number(3), Mult, Number(3), Plus, Number(9), Mult, Number(3), Plus, Hole(Box::new(vec![
                            Number(8), Plus, Number(6), Mult, Number(4)])),])),

                ]
            ),
            ("(2 * 2)", vec![Hole(Box::new(vec![Number(2), Mult, Number(2)]))]),
            ("((2 * 2) + (2 * 2))", vec![
                Hole(Box::new(vec![
                    Hole(Box::new(
                        vec![Number(2), Mult, Number(2)])),
                    Plus,
                    Hole(Box::new(vec![Number(2), Mult, Number(2)]))
                ]))]
            )

            // ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", ),
        ];

        for (input,expected_out) in tests {
            assert_eq!(replace_paren_pair_with_hole(into_raw_expr(input.into())), expected_out);
        }
    }

    #[test]
    fn test_full_expr() {
        let tests = vec![
            ("1 + 2 * 3", 9),
            ("2 * 3 + (4 * 5)", 26),
            // ("5 + (8 * 3 + 9 + 3 * 4 * 3)", ),
            // ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", ),
            // ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", ),
        ];

        for (input,expected_out) in tests {
            assert_eq!(
                expr_eval(
                    replace_paren_pair_with_hole(
                        into_raw_expr(
                            input.into()))), expected_out);
        }
    }
}
