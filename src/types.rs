use std::fmt;

#[derive(Eq, Hash, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Less,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add  => write!(f, "+"),
            Operation::Sub  => write!(f, "-"),
            Operation::Less => write!(f, "<"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    String,
    Bool,
    Function(Box<(Type, Type)>),
}

pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Variable(char),
    Binary(BinExp),
    Conditional(IfExp),
    Function(FunExp),
    Call(CallExp),
}

pub struct BinExp {
    pub left: Box<Expr>,
    pub operator: Operation,
    pub right: Box<Expr>,
}

pub struct IfExp {
    pub condition: Box<Expr>,
    pub then: Box<Expr>,
    pub elze: Box<Expr>,
}

pub struct FunExp {
    pub argument: Box<Expr>,
    pub arg_type: Type,
    pub body: Box<Expr>,
}

pub struct CallExp {
    pub caller: Box<Expr>,
    pub callee: Box<Expr>,
}
