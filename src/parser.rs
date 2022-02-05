use crate::lex::Token;
use crate::lex::Token::{Int, Return, SemiColon};
use crate::parser::Expression::Expr;

// TODO: Not happy with data structure.
// E.g Statement can be program?
enum ASTNode {
    Program,
    FunctionDeclaration,
    Return(Expression),
}

#[derive(PartialEq)]
enum Expression {
    Int(i32),
    Expr(Option<Box<Expression>>)
}

pub(crate) fn construct_ast(tokens: Vec<Token>) {
    println!("hello world")
}

fn parse_expression(tokens: &[Token]) -> Expression {
    let mut expr: Option<Expression> = None;
    for token in tokens {
        match token {
            Token::StartFunction => {}
            Token::EndFunction => {}
            Token::Int => {
                expr = Some(
                    Expression::Int(0)
                )
            }
            Token::Main => {}
            Token::Return => {}
            Token::Number(a) => {}
            Token::SemiColon => {}
        }
    };
    return match expr {
        Some(T) => T,
        None => Expression::Int(0)
    }
}

fn parse_statement<T>(tokens: Vec<Token>) -> ASTNode {
    if tokens[0] != Token::Return {
        panic!("invalid expression")
    }
    if tokens[1] != Token::Int {
    panic!("invalid expression")
    }
    let expr = parse_expression(&tokens[1..]);
    let ret = ASTNode::Return(expr);

    if tokens[2] != Token::SemiColon {
        panic!("invalid expression")
    }
    return ret
}