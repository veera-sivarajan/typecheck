use std::collections::HashMap;
use typecheck::types::{BinExp, CallExp, Expr, FunExp, FunType, IfExp, Operation, Type};

fn test_driver(input: typecheck::types::Expr, output: Result<Type, String>) -> bool {
    let mut context = HashMap::new();
    context.insert('x', Type::Number);
    context.insert('y', Type::Bool);
    context.insert('z', Type::String);
    let mut op_type = HashMap::new();
    op_type.insert(Operation::Add, (Type::Number, Type::Number, Type::Number));
    op_type.insert(Operation::Less, (Type::Number, Type::Number, Type::Number));
    op_type.insert(Operation::Less, (Type::Number, Type::Number, Type::Bool));
    output == typecheck::type_check(&input, &mut context, &op_type)
}

#[test]
fn check_number() {
    let n = Expr::Number(1.0);
    let result = Ok(Type::Number);
    assert!(test_driver(n, result))
}

#[test]
fn check_bool() {
    let n = Expr::Bool(false);
    let result = Ok(Type::Bool);
    assert!(test_driver(n, result))
}

#[test]
fn check_string() {
    let n = Expr::String(format!("Hello, world"));
    let result = Ok(Type::String);
    assert!(test_driver(n, result))
}

#[test]
fn check_variable() {
    let n = Expr::Variable('x');
    let result = Ok(Type::Number);
    assert!(test_driver(n, result))
}

#[test]
fn check_binary() {
    let a = Expr::Number(1.0);
    let b = Expr::Number(2.0);
    let exp = Expr::Binary(BinExp {
        left: Box::new(a),
        right: Box::new(b),
        operator: Operation::Add,
    });
    let result = Ok(Type::Number);
    assert!(test_driver(exp, result))
}

#[test]
fn check_conditional() {
    let a = Expr::Number(1.0);
    let b = Expr::Number(2.0);
    let c = Expr::Bool(false);
    let exp = Expr::Conditional(IfExp {
        condition: Box::new(c),
        then: Box::new(a),
        elze: Box::new(b),
    });
    let result = Ok(Type::Number);
    assert!(test_driver(exp, result))
}

#[test]
fn check_function() {
    let a = Expr::Variable('y');
    let b = Expr::Bool(false);
    let exp = Expr::Function(FunExp {
        argument: Box::new(a),
        arg_type: Type::Bool,
        body: Box::new(b),
    });
    let result = Ok(Type::Function(FunType {
        input: Box::new(Type::Bool),
        output: Box::new(Type::Bool),
    }));
    assert!(test_driver(exp, result))
}

#[test]
fn check_call() {
    let a = Expr::Variable('y');
    let b = Expr::Bool(false);
    let f = Expr::Function(FunExp {
        argument: Box::new(a),
        arg_type: Type::Bool,
        body: Box::new(b),
    });
    let arg = Expr::Variable('y');
    let expr = Expr::Call(CallExp {
        caller: Box::new(f),
        callee: Box::new(arg),
    });
    let result = Ok(Type::Bool);
    assert!(test_driver(expr, result))
}
