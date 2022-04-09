use std::borrow::{Borrow, BorrowMut};
use crate::lex::Token;
use crate::lex::Token::{Int, Return, SemiColon};

use crate::tokens::AstState;
use crate::ast::Tree;

use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use crate::parser::AstState::Function;
use crate::tokens::AstState::Root;

pub(crate) fn construct_ast(tokens: Vec<Token>) -> Tree {
    let mut root = Tree::new();
    let mut ptr = 0;
    for token in tokens {
        match token {
            Token::StartFunction => {
                ptr = root.insert_at(ptr, Function).unwrap();
            }
            Token::EndFunction => {
                ptr = root.parent_handle(ptr);
            }
            Int => {

            }
            Token::Main => {

            }
            Return => {
                ptr = root.insert_at(ptr, AstState::Return).unwrap();
            }
            Token::Number(i) => {
                ptr = root.insert_at(ptr, AstState::Int(i)).unwrap();
            }
            SemiColon => {
                // TODO:
            }
        }
    }
    return root;
}


// fn parse_expression(tokens: &[Token]) -> Expression {
//     let mut expr: Option<Expression> = None;
//     for token in tokens {
//         match token {
//             Token::StartFunction => {}
//             Token::EndFunction => {}
//             Token::Int => {
//                 expr = Some(
//                     Expression::Int(0)
//                 )
//             }
//             Token::Main => {}
//             Token::Return => {}
//             Token::Number(a) => {}
//             Token::SemiColon => {}
//         }
//     };
//     match expr {
//         Some(T) => T,
//         // Failsafe probs should panic
//         None => Expression::Int(0)
//     }
// }
//
// fn parse_statement(tokens: Vec<Token>) -> Expression {
//     if tokens[0] != Token::Return {
//         panic!("invalid expression")
//     }
//     if tokens[1] != Token::Int {
//     panic!("invalid expression")
//     }
//     let expr = parse_expression(&tokens[1..]);
//
//     if tokens[2] != Token::SemiColon {
//         panic!("invalid expression")
//     }
//     return expr
// }