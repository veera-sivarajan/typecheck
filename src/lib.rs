pub mod types;
use crate::types::{BinExp, CallExp, Expr, FunExp, FunType, IfExp, Operation, Type};
use std::collections::HashMap;

pub fn type_check(
    expr: &Expr,
    context: &mut HashMap<char, Type>,
    op_type: &HashMap<Operation, (Type, Type, Type)>,
) -> Result<Type, String> {
    match expr {
        Expr::Number(_) => Ok(Type::Number),
        Expr::String(_) => Ok(Type::String),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Variable(value) => Ok(context.get(value).unwrap().clone()),
        Expr::Binary(value) => {
            let e1 = type_check(&value.left, context, op_type)?;
            let e2 = type_check(&value.right, context, op_type)?;
            let (t1, t2, t3) = op_type.get(&value.operator).unwrap();
            if *t1 == e1 && e2 == *t2 {
                Ok(t3.clone())
            } else {
                Err(format!(
                    "Operands to binary expression '{}' not of expected type.",
                    value.operator
                ))
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
            Ok(Type::Function(FunType {
                input: Box::new(value.arg_type.clone()),
                output: Box::new(t2),
            }))
        }
        Expr::Call(value) => {
            if let Ok(Type::Function(ty)) = type_check(&value.caller, context, op_type) {
                if *ty.input == type_check(&value.callee, context, op_type)? {
                    Ok(*ty.output)
                } else {
                    Err(String::from("Function argument expects a different type."))
                }
            } else {
                Err(String::from("Cannot call from a non-function type."))
            }
        }
    }
}
