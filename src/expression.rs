use bevy::reflect::GetPath;

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::UIState;

use std::str::FromStr;

pub trait Expression<Val>: FromStr + Send + Sync + Clone {
    fn process<T: UIState>(&self, context: &T) -> Val;
}

#[derive(Debug, Clone, Serialize)]
pub struct SimpleExpression(String);

impl FromStr for SimpleExpression {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl<'de> Deserialize<'de> for SimpleExpression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Expression<bool> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> bool {
        let ctx = InternalParser(context);
        evalexpr::eval_boolean_with_context(self.0.as_str(), &ctx).unwrap_or(false)
    }
}

impl Expression<i8> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> i8 {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as i8
    }
}

impl Expression<i32> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> i32 {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as i32
    }
}

impl Expression<i64> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> i64 {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0)
    }
}

impl Expression<i128> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> i128 {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as i128
    }
}

impl Expression<isize> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> isize {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as isize
    }
}

impl Expression<u8> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> u8 {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as u8
    }
}

impl Expression<u32> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> u32 {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as u32
    }
}

impl Expression<u64> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> u64 {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as u64
    }
}

impl Expression<u128> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> u128 {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as u128
    }
}

impl Expression<usize> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> usize {
        let ctx = InternalParser(context);
        evalexpr::eval_int_with_context(self.0.as_str(), &ctx).unwrap_or(0) as usize
    }
}

impl Expression<f32> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> f32 {
        let ctx = InternalParser(context);
        evalexpr::eval_number_with_context(self.0.as_str(), &ctx).unwrap_or(0.) as f32
    }
}

impl Expression<f64> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T) -> f64 {
        let ctx = InternalParser(context);
        evalexpr::eval_number_with_context(self.0.as_str(), &ctx).unwrap_or(0.)
    }
}
pub struct InternalParser<'a, T: UIState>(pub &'a T);

impl<'a, T: UIState> evalexpr::Context for InternalParser<'a, T> {
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

        let Ok(r) = self.0.reflect_path(&path) else {
            return
            evalexpr::EvalexprResult::Err(evalexpr::EvalexprError::FunctionIdentifierNotFound(
                path
            )) };
        if let Some(r) = r.downcast_ref::<bool>() {
            Ok(evalexpr::Value::Boolean(*r))
        } else if let Some(r) = r.downcast_ref::<String>() {
            Ok(evalexpr::Value::String(r.to_string()))
        } else if let Some(r) = r.downcast_ref::<f32>() {
            Ok((*r as f64).into())
        } else if let Some(r) = r.downcast_ref::<f64>() {
            Ok((*r).into())
        } else if let Some(r) = r.downcast_ref::<i8>() {
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<i16>() {
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<i32>() {
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<i64>() {
            Ok((*r).into())
        } else if let Some(r) = r.downcast_ref::<u8>() {
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<u16>() {
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<u32>() {
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<u64>() {
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<isize>() {
            Ok((*r as i64).into())
        } else if let Some(r) = r.downcast_ref::<usize>() {
            Ok((*r as i64).into())
        } else {
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
