use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::{
    ActivationRecord, ActivationRecordType, CallStack,
};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::{Number, TokenEnum},
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct Loop {
    /// an expression
    from_range: ASTNode,
    /// an expression
    to_range: ASTNode,
    step_by: ASTNode,
    block: ASTNode,
}

impl Loop {
    pub fn new(from_range: ASTNode, to_range: ASTNode, step_by: ASTNode, block: ASTNode) -> Self {
        Self {
            from_range,
            to_range,
            step_by,
            block,
        }
    }
}

impl AST for Loop {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        // 1. Visit the from expression, to expression and step expression if they exist. Push
        //    them onto the stack
        //
        // 2. On every loop iteration, we pop these into r0, r1 and r2 and perform the step
        //    operation
        //
        // 3. Compare if the current addition value is equal to the `to` value, and if they are
        //    equal break the loop

        let current_num_loop = asm.num_loops;
        asm.inc_num_loops();

        self.from_range.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
        self.to_range.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
        self.step_by.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);

        call_stack.push("".into(), ActivationRecordType::Loop);

        asm.gen_loop_start(current_num_loop);
        self.block.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
        asm.gen_loop_end(current_num_loop);

        call_stack.pop();
    }

    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        let from = self.from_range.borrow().visit(v, Rc::clone(&f), call_stack);
        let to = self.to_range.borrow().visit(v, Rc::clone(&f), call_stack);
        let step_by = self.step_by.borrow().visit(v, Rc::clone(&f), call_stack);

        if !from.token.is_integer() || !to.token.is_integer() || !step_by.token.is_integer() {
            panic!("Expected from, to and step expressions to be Integer");
        }

        let from = if let TokenEnum::Number(Number::Integer(i)) = *from.token {
            i
        } else {
            unreachable!("Somehow did not get integer even after performing Integer enum check")
        };

        let to = if let TokenEnum::Number(Number::Integer(i)) = *to.token {
            i
        } else {
            unreachable!("Somehow did not get integer even after performing Integer enum check")
        };

        let step_by = if let TokenEnum::Number(Number::Integer(i)) = *step_by.token {
            if i < 0 {
                panic!("Step cannot be negative");
            }

            i as usize
        } else {
            panic!("Step has to be a positive integer")
        };

        for _ in (from..to).step_by(step_by) {
            self.block.borrow().visit(v, Rc::clone(&f), call_stack);
        }

        return VisitResult {
            token: Box::new(TokenEnum::Unknown("".into())),
        };
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        println!("{:#?}", self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        call_stack.push("".into(), ActivationRecordType::Loop);

        self.block.borrow_mut().semantic_visit(call_stack, Rc::clone(&f));
        
        call_stack.pop();
    }
}
