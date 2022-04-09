use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use crate::ast::{Tree, NodeHandle};
use crate::tokens::AstState;

pub(crate) fn generate_asm(root: Tree) {
    let mut f_r = File::create("asm.s");
    match f_r {
        Ok(mut f) => {
            let asm = iterate_tree(&root, 0);
            f.write_all(asm.as_bytes());
            println!("writing AST")
        }
        Err(_) => {}
    }
}

fn iterate_tree(root: &Tree, ptr: NodeHandle) -> String {
    let mut asm = String::new();
    let node = root.at(ptr);
    let children = &node.child;
    for child in children.iter() {
        let node_asm = iterate_tree(root, *child);
        asm.insert_str(0, node_asm.as_str());
    }
    let node_asm: String = match node.data
    {
        AstState::Root => {String::new()}
        AstState::Function => {String::from(" .globl main\nmain:\n")}
        AstState::Statement => {String::new()}
        AstState::Expression => {String::new()}
        AstState::Program => {String::new()}
        AstState::Return => {
            let child = children[0];
            if let AstState::Int(i) = root.at(child).data{
                format!("movl    ${:?}, %eax\nret\n", i)
            } else { panic!("must return int") }
        }
        AstState::SemiColon => {String::from("\n")}
        AstState::Int(i) => {String::new()}
    };
    asm.insert_str(0, node_asm.as_str());
    asm
}
