//! [Rust Design Patterns - Visitor](https://fomalhauthmj.github.io/patterns/patterns/behavioural/visitor.html)
//!
//! # Description
//! 访问器封装了一种在对象的异质集合上操作的算法。
//! 它允许在同一数据上写入多种不同的算法，而不必修改数据（或其主要行为）。
//!

pub mod ast {
    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
    }

    pub struct Name {
        pub value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

pub mod visit {
    use super::ast::*;
    pub trait Visitor {
        type Output: 'static;

        fn visit_name(&mut self, n: &Name) -> Self::Output;
        fn visit_stmt(&mut self, s: &Stmt) -> Self::Output;
        fn visit_expr(&mut self, e: &Expr) -> Self::Output;
    }
}

use ast::*;
use visit::*;

// An example concrete implementation - walks the AST interpreting it as code.
struct Interpreter;

impl Visitor for Interpreter {
    type Output = i64;

    fn visit_name(&mut self, _: &Name) -> i64 {
        0
    }

    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}
