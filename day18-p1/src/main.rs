/*

*/


use regex::Regex;
use clap::{App, Arg};
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RawExpr {
    Number(i64),
    Plus,
    Mult,
    LeftParen,
    RightParen
}

enum ExprElem {
    Number(i64),
    Inner(Box<ExprElem>)
}

enum Operation {
    Plus,
    Mult
}

struct Expr {
    left: ExprElem,
    operation: Operation,
    right: ExprElem
}

fn into_raw_expr(string: String) -> Vec<RawExpr> {
    return string
        .replace("(", "( ")
        .replace(")", " )")
        .split(' ')
        .map(|s| match s {
        "(" => RawExpr::LeftParen,
        ")" => RawExpr::RightParen,
        "+" => RawExpr::Plus,
        "*" => RawExpr::Mult,
        n if n.parse::<i64>().is_ok() => RawExpr::Number(n.parse::<i64>().unwrap()),
        _ => panic!(format!("Unknown symbol {}", s))
    }).collect()
}

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

    let lines: Vec<String> = input.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    for line in lines {
        println!("{} -> {:?}", line.clone(), into_raw_expr(line));
    }
}


#[cfg(test)]
mod test_expr {
    use super::*;
    use RawExpr::*;

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
    fn test_compress() {
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
}
