use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::{TokenEnum, VariableEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct AssignmentStatement {
    var_name: String,
    right: ASTNode,
}

impl AssignmentStatement {
    pub fn new(var_name: String, right: ASTNode) -> Self {
        Self { var_name, right }
    }
}

impl AST for AssignmentStatement {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        self.right.borrow().visit_com(v, f, asm, call_stack);
        asm.variable_assignment(&self.var_name, call_stack);
    }

    // TODO: change this so that the expression is stored here and we need to visit the varible
    // to evaluate the value
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        let right_visit = self.right.borrow().visit(v, f, call_stack);

        match &*right_visit.token {
            TokenEnum::StringLiteral(s) => {
                v.insert(self.var_name.clone(), VariableEnum::String(s.into()));
            }

            TokenEnum::Number(n) => {
                v.insert(self.var_name.clone(), VariableEnum::Number(n.clone()));
            }

            TokenEnum::Variable(_) => todo!(),

            _ => {
                panic!("Variable value is not a Number, String or Variable");
            }
        }

        return VisitResult {
            token: right_visit.token,
        };
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        println!("{:#?}", self)
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, _f: Rc<RefCell<Functions>>) {
        if !call_stack.var_with_name_found(&self.var_name) {
            panic!("Variable '{}' not found in current scope", &self.var_name);
        }
    }
}
