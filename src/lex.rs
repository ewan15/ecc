use std::fs;
use std::ops::Index;
use regex::{Match, Regex};
use crate::lex::Token::{EndFunction, Int, Number, Return, SemiColon, StartFunction};

#[derive(PartialEq)]
pub(crate) enum Token {
    StartFunction,
    EndFunction,
    Int,
    Main,
    Return,
    Number(i32),
    SemiColon
}

fn string_to_token(token_str: &str) -> Token {
    return match token_str {
        "{" => StartFunction,
        "}" => EndFunction,
        "int" => Int,
        "return" => Return,
        ";" => SemiColon,
        &_ => { Number(5) }
    }
}

pub(crate) fn parse(file_location: String) -> Vec<Token> {
    let contents = fs::read_to_string(file_location)
        .expect("Something went wrong reading the file").replace('\n', "");
    let re = Regex::new(r"\{|}|int|return|[0-9]|;").unwrap();
    let mat = re.captures_iter(&*contents);
    let mut tokens: Vec<Token> = Vec::new();

    for m in mat {
        let token_str = m.get(0).unwrap().as_str();
        println!("{}",token_str);
        let token = string_to_token(token_str);
        tokens.push(token);
    };

    tokens
}
