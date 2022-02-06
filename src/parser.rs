use std::borrow::{Borrow, BorrowMut};
use crate::lex::Token;
use crate::lex::Token::{Int, Return, SemiColon};
use crate::parser::ASTNode::{Function, Program, Statement};
use crate::parser::Expression::Expr;

use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};

pub(crate) type Node = Option<Rc<RefCell<ASTNode>>>;

// TODO: Not happy with data structure.
// E.g Statement can be program?
pub enum ASTNode {
    Function(Node),
    Statement(Option<Rc<RefCell<Expression>>>),
    Expression(Expression),
    Program(Node),
}

#[derive(PartialEq)]
pub enum Expression {
    Int(i32),
    Expr(Option<Rc<Expression>>)
}

pub(crate) fn construct_ast(tokens: Vec<Token>) -> Node {
    let mut root = Rc::new(RefCell::new(
        Program(
            None
        )));
    let mut child: Rc<RefCell<ASTNode>> = Rc::clone(&root);
    let mut start_function_i = 0;
    for x in 0..tokens.len() {
        match tokens[x] {
            Token::StartFunction => {
                start_function_i = x;
            }
            Token::EndFunction => {
                // This should really be in main
                let new_child = Rc::clone(&child);
                match (*new_child).borrow_mut().deref_mut() {
                    ASTNode::Function(_) => {}
                    ASTNode::Statement(_) => {}
                    ASTNode::Expression(_) => {}
                    ASTNode::Program(ptr) => {
                        let new_child = Rc::new(RefCell::new(
                            Function(None)
                        ));
                        child = Rc::clone(&new_child);
                        ptr.replace(new_child);
                    }
                };
                let expr = parse_expression(&tokens[start_function_i..x]);
                let new_child = Rc::clone(&child);
                match (*new_child).borrow_mut().deref_mut() {
                    ASTNode::Function(ptr) => {
                        let statement = Rc::new(RefCell::new(
                            Statement(Some(
                                Rc::new(RefCell::new(
                                    expr
                                ))
                            ))
                        ));
                        ptr.replace(statement);
                    }
                    a => {
                        println!("hi")
                    }
                };
            }
            Int => {}
            Token::Main => {}
            Return => {}
            Token::Number(i) => {}
            SemiColon => {}
        }
    }
    Some(root)
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
    match expr {
        Some(T) => T,
        // Failsafe probs should panic
        None => Expression::Int(0)
    }
}

fn parse_statement(tokens: Vec<Token>) -> Expression {
    if tokens[0] != Token::Return {
        panic!("invalid expression")
    }
    if tokens[1] != Token::Int {
    panic!("invalid expression")
    }
    let expr = parse_expression(&tokens[1..]);

    if tokens[2] != Token::SemiColon {
        panic!("invalid expression")
    }
    return expr
}