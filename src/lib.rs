mod types;
use crate::types::{Type, Operation, Expr, BinExp, IfExp, FunExp, CallExp};
use std::collections::HashMap;

fn type_check(
    expr: &Expr,
    context: &mut HashMap<char, Type>,
    op_type: &HashMap<Operation, (Type, Type, Type)>,
) -> Result<Type, String> {
    match expr {
        Expr::Number(_)       => Ok(Type::Number),
        Expr::String(_)       => Ok(Type::String),
        Expr::Bool(_)         => Ok(Type::Bool),
        Expr::Variable(value) => Ok(context.get(value).unwrap().clone()),
        Expr::Binary(value)   => {
            let e1 = type_check(&value.left, context, op_type)?;
            let e2 = type_check(&value.right, context, op_type)?;
            let (t1, t2, t3) = op_type.get(&value.operator).unwrap();
            if *t1 == e1 && e2 == *t2 {
                Ok(t3.clone())
            } else {
                Err(format!("Operands to binary expression '{}' not of expected type.", value.operator))
            }
        }
        Expr::Conditional(value) => {
            let Ok(Type::Bool) = type_check(&value.condition, context, op_type) else {
                return Err(String::from("Conditional expression not of type bool."));
            };
            let t2 = type_check(&value.then, context, op_type)?;
            let t3 = type_check(&value.elze, context, op_type)?;
            if t2 == t3 {
                Ok(t3)
            } else {
                Err(String::from("Then and else branch should be of same type."))
            }
        }
        Expr::Function(value) => {
            let Expr::Variable(var) = *value.argument else {
                return Err(String::from("Argument should be a variable."));
            };
            context.insert(var, value.arg_type.clone());
            let t2 = type_check(&value.body, context, op_type)?;
            Ok(Type::Function(Box::new((value.arg_type.clone(), t2))))
        }
        Expr::Call(value) => {
            if let Ok(Type::Function(ty)) = type_check(&value.caller, context, op_type) {
                if ty.0 == type_check(&value.callee, context, op_type)? {
                    Ok(ty.1)
                } else {
                    Err(String::from("Function argument expects a different type."))
                }
            } else {
                Err(String::from("Cannot call from a non-function type."))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_driver(input: Expr, output: Result<Type, String>) -> bool {
        let mut context = HashMap::new();
        context.insert('x', Type::Number);
        context.insert('y', Type::Bool);
        context.insert('z', Type::String);
        let mut op_type = HashMap::new();
        op_type.insert(Operation::Add, (Type::Number, Type::Number, Type::Number));
        op_type.insert(Operation::Less, (Type::Number, Type::Number, Type::Number));
        op_type.insert(Operation::Less, (Type::Number, Type::Number, Type::Bool));
        output == type_check(&input, &mut context, &op_type)
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
        let result = Ok(Type::Function(Box::new((Type::Bool, Type::Bool))));
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
}
