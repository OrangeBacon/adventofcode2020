use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};
use logos::{Logos, Lexer};
use std::iter::Peekable;

fn get_next_value(c: char, input: &mut dyn Iterator<Item = char>) -> i64 {
    if c == '(' {
        evaluate(input)
    } else {
        c.to_digit(10).expect(&format!("{}", c)) as i64
    }
}

fn evaluate(input: &mut dyn Iterator<Item = char>) -> i64 {
    let mut acc = get_next_value(input.next().unwrap(), input);

    while let Some(c) = input.next() {
        match c {
            '+' => acc += get_next_value(input.next().unwrap(), input),
            '*' => acc *= get_next_value(input.next().unwrap(), input),
            ')' => break,
            _ => panic!(),
        }
    }

    acc
}

fn parse_num(lex: &mut Lexer<Token>) -> Option<i64> {
    Some(lex.slice().chars().next()?.to_digit(10)? as i64)
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Token {
    #[regex("[0-9]", parse_num)]
    Number(i64),

    #[token("*")]
    Mult,

    #[token("+")]
    Add,

    #[token("(")]
    Open,

    #[token(")")]
    Close,

    #[error]
    #[regex(" ", logos::skip)]
    Error,
}

impl Token {
    fn prec(self) -> u32 {
        use Token::*;
        match self {
            Mult => 1,
            Add => 2,
            Number(_) => 3,
            _ => 0,
        }
    }

    fn prefix(self, lex: &mut Peekable<Lexer<Token>>) -> i64 {
        use Token::*;
        match self {
            Number(a) => a,
            Open => {
                let r = parse_precidence(lex, 1);
                lex.next();
                r
            }
            _ => panic!(),
        }
    }

    fn infix(self, lex: &mut Peekable<Lexer<Token>>, prev: i64) -> i64 {
        use Token::*;
        match self {
            Mult => {
                prev * parse_precidence(lex, 2)
            }
            Add => {
                prev + parse_precidence(lex, 3)
            }
            a => panic!(format!("{:?}", a)),
        }
    }
}

/*enum Precidence {
    None,
    Mult,
    Add,
    Primary,
}*/

fn parse_precidence(lex: &mut Peekable<Lexer<Token>>, precidence: u32) -> i64 {
    let tok = lex.next().unwrap();
    let mut acc = tok.prefix(lex);

    loop {
        if lex.peek().is_none() {break;}
        if precidence > lex.peek().unwrap().prec() {break;}
        acc = lex.next().unwrap().infix(lex, acc);
    }

    acc
}

#[aoc("5019432542701", "70518821989947")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    timer.lap("Parse");


    let part1 = input.lines().fold(0, |a,b|a+evaluate(&mut b.chars().filter(|&x|x!=' ')));
    timer.lap("Part 1");

    let part2 = input.lines().fold(0, |a,b| a + parse_precidence(&mut Token::lexer(b).peekable(), 1));
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
