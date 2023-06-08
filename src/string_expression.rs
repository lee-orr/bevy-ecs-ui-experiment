use std::str::FromStr;

use bevy::math::bool;

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::{expression::*, UIState};

#[derive(Debug, Clone, Serialize)]
pub enum StringExpression {
    Value(String),
    Expression(Vec<(String, bool)>),
}

impl Expression<String> for StringExpression {
    fn process<T: UIState>(&self, context: &T) -> String {
        match self {
            StringExpression::Value(s) => s.to_owned(),
            StringExpression::Expression(v) => {
                let len = v.len();
                let ctx = InternalParser(context);
                v.iter()
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
    }
}

impl FromStr for StringExpression {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains("{{") {
            Ok(Self::Value(s.to_string()))
        } else {
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
            Ok(Self::Expression(values))
        }
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
        let StringExpression::Value(expression) = StringExpression::from_str("This has no expressions 1 + 2!").unwrap() else { panic!("Parsed as an expression")};
        assert_eq!(expression, "This has no expressions 1 + 2!");
    }

    #[test]
    fn parses_a_string_with_an_expression_correctly() {
        let StringExpression::Expression(expression) = StringExpression::from_str("This has expressions {{1 + 2}}!").unwrap() else { panic!("Parsed as a string")};
        assert_eq!(expression.len(), 3);
        assert_eq!(expression.get(0).unwrap().0, "This has expressions ");
        assert!(!expression.get(0).unwrap().1);
        assert_eq!(expression.get(1).unwrap().0, "1 + 2");
        assert!(expression.get(1).unwrap().1);
        assert_eq!(expression.get(2).unwrap().0, "!");
        assert!(!expression.get(2).unwrap().1);
    }

    #[test]
    fn returns_a_non_expression_correctly() {
        let state = State {
            number: 5,
            string: "Test".to_string(),
        };

        let expression = StringExpression::Value(
            "(number() + 2).to_string() + \" is the result of the \" + string()".to_string(),
        );
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

        let expression = StringExpression::Expression(vec![(
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