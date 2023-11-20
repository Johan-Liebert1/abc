use crate::{
    constants,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{Comparators, Number, Operand, TokenEnum},
    },
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct ComparisonExp {
    left: Box<dyn AST>,
    comp_op: Box<Token>,
    right: Box<dyn AST>,
}

impl ComparisonExp {
    pub fn new(left: Box<dyn AST>, comp_op: Box<Token>, right: Box<dyn AST>) -> Self {
        Self {
            left,
            comp_op,
            right,
        }
    }

    fn compare<T>(&self, l: T, r: T) -> TokenEnum
    where
        T: PartialOrd,
    {
        return TokenEnum::Bool(match &self.comp_op.token {
            TokenEnum::Comparator(comp) => match comp {
                Comparators::LessThan => l < r,
                Comparators::GreaterThan => l > r,
                Comparators::LessThanEq => l <= r,
                Comparators::GreaterThanEq => l >= r,
            },

            _ => {
                unreachable!("Found non comparator")
            }
        });
    }

    fn eval_number_number(&self, left_op: &Number, right_op: &Number) -> VisitResult {
        match (left_op, right_op) {
            (Number::Integer(l), Number::Integer(r)) => VisitResult {
                token: Box::new(self.compare(*l, *r)),
            },

            (Number::Float(l), Number::Float(r)) => VisitResult {
                token: Box::new(self.compare(*l, *r)),
            },

            _ => {
                panic!("Cannot compare Float and Integer");
            }
        }
    }

    fn eval_var_num(&self, number: &Number, variable: &String, i: &mut Variables) -> VisitResult {
        let result = i.get(variable);

        match result {
            Some(var_num) => self.eval_number_number(number, var_num),

            None => panic!("Variable {} is not defined", variable),
        }
    }

    fn eval_var_var(&self, var1: &String, var2: &String, i: &mut Variables) -> VisitResult {
        let r1 = i.get(var1);
        let r2 = i.get(var2);

        match (r1, r2) {
            (Some(var1), Some(var2)) => self.eval_number_number(var1, var2),

            (None, Some(_)) => panic!("Variable {} is not defined", var1),
            (Some(_), None) => panic!("Variable {} is not defined", var2),
            (None, None) => panic!("Variable {} and {} is not defined", var1, var2),
        }
    }

    fn evaluate_operands(
        &self,
        left_op: &Operand,
        right_op: &Operand,
        i: &mut Variables,
    ) -> VisitResult {
        match (left_op, right_op) {
            (Operand::Number(left_op), Operand::Number(right_op)) => {
                self.eval_number_number(left_op, right_op)
            }

            (Operand::Number(n), Operand::Variable(v)) => self.eval_var_num(n, v, i),
            (Operand::Variable(v), Operand::Number(n)) => self.eval_var_num(n, v, i),

            (Operand::Variable(v1), Operand::Variable(v2)) => self.eval_var_var(v1, v2, i),
        }
    }
}

impl AST for ComparisonExp {
    fn visit(&self, i: &mut Variables, f: Rc<RefCell<&Functions>>) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:#?}", &self);
            println!("===============================================");
        }

        let visit_left = self.left.visit(i, Rc::clone(&f));
        let visit_right = self.right.visit(i, Rc::clone(&f));

        let left_operand = visit_left.token.get_operand();
        let right_operand = visit_right.token.get_operand();

        match (&left_operand, &right_operand) {
            (Ok(lop), Ok(rop)) => {
                // Handle the case where both operands are Ok
                return self.evaluate_operands(lop, rop, i);
            }

            (Err(err), _) => {
                // Handle the case where left_operand is an error
                panic!("{}", err);
            }

            (_, Err(err)) => {
                // Handle the case where right_operand is an error
                panic!("{}", err);
            }
        };
    }

    fn get_token(&self) -> &Token {
        return &self.comp_op;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
