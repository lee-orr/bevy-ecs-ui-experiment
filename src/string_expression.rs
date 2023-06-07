use std::str::FromStr;

use bevy::{math::bool, reflect::GetPath};
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
                    let str = evalexpr::eval_with_context(expr.as_str(), &ctx);
                    if let Err(e) = &str {
                        eprintln!("Eval Error: {e:+?}");
                    }
                    let val = str.unwrap_or(evalexpr::Value::Empty);
                    let val = match &val {
                        evalexpr::Value::String(s) => s.to_string(),
                        _ => val.to_string(),
                    };
                    v.push(val);
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
        let values = s
            .split("}}")
            .flat_map(|v| {
                let Some((before, after)) = v.split_once("{{") else {
                return [Some((v.to_string(), false)), None];
            };
                [
                    Some((before.to_string(), false)),
                    Some((after.to_string(), true)),
                ]
            })
            .flatten()
            .collect();
        Ok(Self(values))
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

    #[derive(Component, Reflect, Default)]
    struct State {
        number: i32,
        string: String,
    }

    #[test]
    fn parses_a_string_without_an_expression_correctly() {
        let expression = StringExpression::from_str("This has no expressions 1 + 2!").unwrap();
        assert_eq!(expression.0.len(), 1);
        assert_eq!(
            expression.0.get(0).unwrap().0,
            "This has no expressions 1 + 2!"
        );
        assert!(!expression.0.get(0).unwrap().1);
    }

    #[test]
    fn parses_a_string_with_an_expression_correctly() {
        let expression = StringExpression::from_str("This has expressions {{1 + 2}}!").unwrap();
        assert_eq!(expression.0.len(), 3);
        assert_eq!(expression.0.get(0).unwrap().0, "This has expressions ");
        assert!(!expression.0.get(0).unwrap().1);
        assert_eq!(expression.0.get(1).unwrap().0, "1 + 2");
        assert!(expression.0.get(1).unwrap().1);
        assert_eq!(expression.0.get(2).unwrap().0, "!");
        assert!(!expression.0.get(2).unwrap().1);
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

    #[test]
    fn processes_a_parsed_string_correctly() {
        let state = State::default();
        let expression = StringExpression::from_str("This has expressions {{1 + 2}}!").unwrap();
        let result = expression.process(&state);
        assert_eq!(result, "This has expressions 3!");
    }
}
