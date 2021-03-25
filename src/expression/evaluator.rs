use super::lexer::S;
use super::{math, Expression};
use std::rc::Rc;
use std::str::FromStr;

// Convert a char to an f32 and panic if it doesn't fit
fn tof32(c: char) -> f32 {
    f32::from_str(&c.to_string()).unwrap()
}

pub fn evaluate(expression: Expression) -> f32 {
    solve(expression.s)
}

fn solve(s: S) -> f32 {
    match s {
        S::Atom(c) => tof32(c),
        S::Cons(head, (left, right)) => {
            let left = match Rc::try_unwrap(left.expect("unexpected empty expression")).unwrap() {
                S::Atom(c) => tof32(c),
                s => solve(s),
            };
            let right = match right {
                Some(s) => match Rc::try_unwrap(s).unwrap() {
                    S::Atom(c) => tof32(c),
                    s => solve(s),
                },
                None => evaluate_single(head, left),
            };

            evaluate_equation(left, head, right)
        }
    }
}

/// Evaluate {left} {operator} {right} (e.g 4 + 3)
fn evaluate_equation(left: f32, operator: char, right: f32) -> f32 {
    match operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        t => panic!("unknown operator: {:?}", t),
    }
}

fn evaluate_single(operator: char, value: f32) -> f32 {
    match operator {
        '!' => math::factorial(value as usize) as f32,
        '-' => value * -1f32,
        t => panic!("unknown singular operator: {:?}", t),
    }
}
