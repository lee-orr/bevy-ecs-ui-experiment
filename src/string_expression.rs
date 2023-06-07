use std::str::FromStr;

use bevy::{
    math::bool,
    reflect::{GetPath},
};
use evalexpr::Context;
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::UIState;

#[derive(Debug, Clone, Serialize)]
pub struct StringExpression(Vec<(String, bool)>);

impl StringExpression {
    pub fn process<T: UIState>(&self, context: &T) -> String {
        let len = self.0.len();
        let ctx = InternalParser(context);
        self.0
            .iter()
            .fold(Vec::with_capacity(len), |mut v, (expr, is_expr)| {
                if *is_expr {
                    let str = evalexpr::eval_string_with_context(expr.as_str(), &ctx);
                    if let Err(e) = &str {
                        eprintln!("Eval Error: {e:+?}");
                    }
                    v.push(str.unwrap_or_default());
                } else {
                    v.push(expr.to_string());
                }
                v
            })
            .join("")
    }
}

impl FromStr for StringExpression {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(vec![(s.to_string(), false)]))
    }
}

impl<'de> Deserialize<'de> for StringExpression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

struct InternalParser<'a, T: UIState>(&'a T);

impl<'a, T: UIState> Context for InternalParser<'a, T> {
    fn get_value(&self, _identifier: &str) -> Option<&evalexpr::Value> {
        None
    }

    fn call_function(
        &self,
        identifier: &str,
        argument: &evalexpr::Value,
    ) -> evalexpr::EvalexprResult<evalexpr::Value> {
        let path = match argument.as_string() {
            Ok(v) => format!("{identifier}.{v}"),
            Err(_) => identifier.to_string(),
        };
        println!("Parsing Expression at path: {path}");
        let Ok(r) = self.0.reflect_path(&path) else {
            eprintln!("No such path");
            return
            evalexpr::EvalexprResult::Err(evalexpr::EvalexprError::FunctionIdentifierNotFound(
                path
            )) };
        if let Some(r) = r.downcast_ref::<bool>() {
            println!("got a bool {r}");
            Ok(evalexpr::Value::Boolean(*r))
        } else if let Some(r) = r.downcast_ref::<String>() {
            println!("got a string {r}");
            Ok(evalexpr::Value::String(r.to_string()))
        } else if let Some(r) = r.downcast_ref::<f32>() {
            println!("got a float {r}");
            Ok((*r as f64).into())
        } else if let Some(r) = r.downcast_ref::<f64>() {
            println!("got a float {r}");
            Ok((*r).into())
        } else if let Some(r) = r.downcast_ref::<i8>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<i16>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<i32>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<i64>() {
            println!("got an int {r}");
            Ok((*r).into())
        } else if let Some(r) = r.downcast_ref::<u8>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<u16>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<u32>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<u64>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<isize>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<usize>() {
            println!("got an int {r}");
            Ok((*r as i64).into())
        } else {
            eprintln!("Couldn't process state");
            evalexpr::EvalexprResult::Err(evalexpr::EvalexprError::CustomMessage(
                "couldn't process state".to_string(),
            ))
        }
    }

    fn are_builtin_functions_disabled(&self) -> bool {
        false
    }

    fn set_builtin_functions_disabled(&mut self, _disabled: bool) -> evalexpr::EvalexprResult<()> {
        evalexpr::EvalexprResult::Err(evalexpr::EvalexprError::ContextNotMutable)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::prelude::*;

    #[derive(Component, Reflect)]
    struct State {
        number: i32,
        string: String,
    }

    #[test]
    fn returns_a_non_expression_correctly() {
        let state = State {
            number: 5,
            string: "Test".to_string(),
        };

        let expression = StringExpression(vec![(
            "(number() + 2).to_string() + \" is the result of the \" + string()".to_string(),
            false,
        )]);
        let result = expression.process(&state);

        assert_eq!(
            result,
            "(number() + 2).to_string() + \" is the result of the \" + string()"
        );
    }

    #[test]
    fn parses_and_computes_a_simple_expression() {
        let state = State {
            number: 5,
            string: "Test".to_string(),
        };

        let expression = StringExpression(vec![(
            "str::from(number() + 2) + \" is the result of the \" + string()".to_string(),
            true,
        )]);
        let result = expression.process(&state);

        assert_eq!(result, "7 is the result of the Test");
    }
}
