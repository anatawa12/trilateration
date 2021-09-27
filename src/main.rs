mod builder;
mod parser;
mod point;
mod scope;

use crate::builder::Builder;
use crate::parser::IdOrNum::{Id, Num};
use crate::parser::{parse_header_line, parse_line};
use crate::point::Point;
use crate::scope::Scope;
use std::env::args;
use std::io::BufRead;

fn main() {
    let mut arg = args();
    arg.next();

    let mut bd = Builder::new();

    let mut scope = Scope::new();

    let stdin = std::io::stdin();
    let stdin = std::io::BufReader::new(stdin);
    let mut lines = stdin.lines().enumerate();

    let (origin_x, origin_y, width, height, scale) = parse_header_line(
        &lines
            .next()
            .map(|x| x.1)
            .and_then(Result::ok)
            .expect("expect header line"),
    );

    for (num, line) in lines {
        let line = line.unwrap();
        let line = line.trim();
        if line.chars().nth(0) == Some('#') || line.is_empty() {
            continue;
        }

        let line = parse_line(&line, num);
        let res = match line.func {
            "xy" => {
                if let &[Num(x), Num(y)] = line.args.as_slice() {
                    bd.xy(x, y)
                } else {
                    panic!("invalid args for xy(i32, i32)")
                }
            }
            "line" => {
                if let &[Id(x), Id(y)] = line.args.as_slice() {
                    bd.line(scope.resolve(x), scope.resolve(y));
                    Point::ORIGIN
                } else {
                    panic!("invalid args for line(point, point)")
                }
            }
            "lplp" => {
                if let &[Num(l1), Id(p1), Num(l2), Id(p2)] = line.args.as_slice() {
                    bd.lplp(l1, scope.resolve(p1), l2, scope.resolve(p2))
                } else {
                    panic!("invalid args for lplp(i32, point, i32, point)")
                }
            }
            _ => panic!("unknown func: {}", line.func),
        };
        if let Some(assign_to) = line.assign_to {
            scope.assign(assign_to, res);
        }
    }

    bd.print(
        Point(origin_x as f64, origin_y as f64),
        width,
        height,
        scale as f64,
    );
}
