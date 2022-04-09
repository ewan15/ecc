use std::fmt;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Formatter, write};
use std::rc::Rc;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum AstState {
    Root,
    Function,
    Statement,
    Expression,
    Program,
    Return,
    SemiColon,
    Int(i32)
}

impl fmt::Display for AstState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}
