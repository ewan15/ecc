use std::fs;
use std::ops::Index;
use regex::{Match, Regex};
use crate::lex::Token::{EndFunction, Int, Number, Return, StartFunction};

enum Token {
    StartFunction,
    EndFunction,
    Int,
    Main,
    Return,
    Number
}

fn string_to_token(token_str: &str) -> Token {
    return match token_str {
        "{" => StartFunction,
        "}" => EndFunction,
        "int" => Int,
        "main" => Return,
        &_ => { Number }
    }
}

pub(crate) fn parse(file_location: String) -> Vec<Token> {
    let contents = fs::read_to_string(file_location)
        .expect("Something went wrong reading the file").replace('\n', "");
    let re = Regex::new(r"\{|}|int|main|return|[0-9]").unwrap();
    let mat = re.captures_iter(&*contents);
    let mut tokens: Vec<Token> = Vec::new();

    for m in mat {
        let token = string_to_token(m.get(0).unwrap().as_str());
        tokens.push(token);
    };

    tokens
}
