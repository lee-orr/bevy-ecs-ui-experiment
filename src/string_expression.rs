use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::{expression::*, UIState};

#[derive(Debug, Clone, Serialize)]
pub enum StringExpression {
    Value(String),
    Expression(Vec<ExpressionSection>),
}

#[derive(Debug, Clone, Serialize)]
pub enum ExpressionSection {
    Value(String),
    Expression(RawExpression),
}

impl Expression<String> for StringExpression {
    fn process<T: UIState>(&self, context: &T) -> String {
        match self {
            StringExpression::Value(s) => s.to_owned(),
            StringExpression::Expression(v) => {
                let len = v.len();
                let ctx = InternalParser(context);
                v.iter()
                    .fold(Vec::with_capacity(len), |mut v, expo| {
                        match expo {
                            ExpressionSection::Expression(expr) => {
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
                            }
                            ExpressionSection::Value(expr) => {
                                v.push(expr.to_string());
                            }
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
                    return [Some(ExpressionSection::Value(v.to_string())), None];
                };
                    [
                        Some(ExpressionSection::Value(before.to_string())),
                        RawExpression::from_str(after)
                            .ok()
                            .map(ExpressionSection::Expression),
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

        let ExpressionSection::Value(v) = expression.get(0).unwrap() else { panic!("Should start with a value")};
        assert_eq!(v.as_str(), "This has expressions ");

        let ExpressionSection::Expression(v) = expression.get(1).unwrap() else { panic!("Should contain an expression")};
        assert_eq!(v.as_str(), "1 + 2");

        let ExpressionSection::Value(v) = expression.get(2).unwrap() else { panic!("Should end with a value")};
        assert_eq!(v.as_str(), "!");
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

        let expression = StringExpression::Expression(vec![ExpressionSection::Expression(
            RawExpression::from_str(
                "str::from(number() + 2) + \" is the result of the \" + string()",
            )
            .unwrap(),
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
