use bevy::{
    prelude::{info, Resource},
    reflect::{GetPath, TypeUuid},
};

use rhai::{Engine, EvalAltResult, Position};
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::{ExpressionValue, UIState};

use std::{marker::PhantomData, str::FromStr};

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "a84df920-9542-4e4b-8b2e-25601c9d5003"]
pub struct RawExpression(pub rhai::AST);

impl FromStr for RawExpression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        rhai::Engine::new()
            .compile_expression(s)
            .map(|mut ast| {
                ast.set_source(s);
                ast
            })
            .map(Self)
            .map_err(|o| o.to_string())
    }
}

impl<'de> Deserialize<'de> for RawExpression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl ToString for RawExpression {
    fn to_string(&self) -> String {
        self.0.source().unwrap_or("").to_string()
    }
}

impl Serialize for RawExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Some(source) = self.0.source() else {
            return serializer.serialize_str("");
        };
        serializer.serialize_str(source)
    }
}
impl RawExpression {
    pub fn as_str(&self) -> &str {
        self.0.source().unwrap_or("")
    }
}

#[derive(Resource, Debug)]
pub struct ExpressionEngine<T: UIState>(rhai::Engine, PhantomData<T>);

impl<T: UIState> ExpressionEngine<T> {
    pub fn new() -> Self {
        let mut rhai = Engine::new();
        rhai.register_type_with_name::<UIStateHandle<T>>("State")
            .register_indexer_get(UIStateHandle::<T>::get_ui_state_field);
        Self(rhai, Default::default())
    }

    pub fn process_expression(
        &self,
        context: &T,
        expression: &RawExpression,
    ) -> Result<ExpressionValue, String> {
        let mut rhai = Engine::new();
        rhai.register_type_with_name::<&T>("State")
            .register_indexer_get(UIStateHandle::<T>::get_ui_state_field);
        let handle = UIStateHandle::from(context);
        rhai.on_var(move |name, _, _| {
            let handle = handle.clone();
            if name == "state" {
                eprintln!("Requested state");
                Ok(Some(rhai::Dynamic::from(handle)))
            } else {
                Ok(None)
            }
        });
        rhai.eval_ast(&expression.0).map_err(|p| p.to_string())
    }
}

impl<T: UIState> Default for ExpressionEngine<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Expression<Val>: Send + Sync + Clone {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> Val;
}

#[derive(Debug, Clone, Serialize)]
pub struct SimpleExpression(pub RawExpression);

#[derive(Debug, Clone)]
pub struct ArrayExpression(pub Vec<SimpleExpression>);

impl Expression<Option<usize>> for ArrayExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> Option<usize> {
        info!("Processing Array Expression {self:?}");
        for (id, exp) in self.0.iter().enumerate() {
            if exp.process(context, engine) {
                info!("Got true at {id} - {exp:?}");
                return Some(id);
            }
        }
        info!("Got false");
        None
    }
}

impl FromStr for SimpleExpression {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(RawExpression::from_str(s)?))
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
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> bool {
        match engine
            .process_expression(context, &self.0)
            .map(|v| v.as_bool())
        {
            Ok(Ok(v)) => v,
            _ => false,
        }
    }
}

fn process_int<T: UIState>(context: &T, engine: &ExpressionEngine<T>, exp: &RawExpression) -> i64 {
    match engine.process_expression(context, exp).map(|v| v.as_int()) {
        Ok(Ok(v)) => v,
        _ => 0,
    }
}

fn process_uint<T: UIState>(context: &T, engine: &ExpressionEngine<T>, exp: &RawExpression) -> i64 {
    match engine.process_expression(context, exp).map(|v| v.as_int()) {
        Ok(Ok(v)) => v,
        _ => 0,
    }
}

impl Expression<i8> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> i8 {
        process_int(context, engine, &self.0) as i8
    }
}

impl Expression<i32> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> i32 {
        process_int(context, engine, &self.0) as i32
    }
}

impl Expression<i64> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> i64 {
        process_int(context, engine, &self.0)
    }
}

impl Expression<i128> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> i128 {
        process_int(context, engine, &self.0) as i128
    }
}

impl Expression<isize> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> isize {
        process_int(context, engine, &self.0) as isize
    }
}

impl Expression<u8> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> u8 {
        process_uint(context, engine, &self.0) as u8
    }
}

impl Expression<u32> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> u32 {
        process_uint(context, engine, &self.0) as u32
    }
}

impl Expression<u64> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> u64 {
        process_uint(context, engine, &self.0) as u64
    }
}

impl Expression<u128> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> u128 {
        process_uint(context, engine, &self.0) as u128
    }
}

impl Expression<usize> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> usize {
        process_uint(context, engine, &self.0) as usize
    }
}

impl Expression<f32> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> f32 {
        match engine
            .process_expression(context, &self.0)
            .map(|v| v.as_float())
        {
            Ok(Ok(v)) => v as f32,
            _ => 0.,
        }
    }
}

impl Expression<f64> for SimpleExpression {
    fn process<T: UIState>(&self, context: &T, engine: &ExpressionEngine<T>) -> f64 {
        match engine
            .process_expression(context, &self.0)
            .map(|v| v.as_float())
        {
            Ok(Ok(v)) => v,
            _ => 0.,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct UIStateHandle<T: UIState>(usize, PhantomData<T>);

impl<T: UIState> UIStateHandle<T> {
    pub fn new(state: &T) -> Self {
        let handle = unsafe { std::mem::transmute(state) };
        Self(handle, Default::default())
    }

    pub fn static_ref(&self) -> &'static T {
        unsafe { std::mem::transmute(self.0) }
    }

    pub fn get_ui_state_field(
        &mut self,
        index: String,
    ) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
        eprintln!("Handling state field at {index:?}");
        let ctx = self.static_ref();
        eprintln!("State Value: {ctx:+?}");

        let Ok(r) = ctx.reflect_path(&index) else {
            eprintln!("Nothing at path {index}");
            return Err(Box::new(EvalAltResult::ErrorPropertyNotFound(index, Position::NONE)));
        };
        let result = if let Some(r) = r.downcast_ref::<bool>() {
            rhai::Dynamic::from(*r)
        } else if let Some(r) = r.downcast_ref::<String>() {
            rhai::Dynamic::from(r.to_string())
        } else if let Some(r) = r.downcast_ref::<f32>() {
            (*r as f64).into()
        } else if let Some(r) = r.downcast_ref::<f64>() {
            (*r).into()
        } else if let Some(r) = r.downcast_ref::<i8>() {
            (*r as i64).into()
        } else if let Some(r) = r.downcast_ref::<i16>() {
            (*r as i64).into()
        } else if let Some(r) = r.downcast_ref::<i32>() {
            (*r as i64).into()
        } else if let Some(r) = r.downcast_ref::<i64>() {
            (*r).into()
        } else if let Some(r) = r.downcast_ref::<u8>() {
            (*r as i64).into()
        } else if let Some(r) = r.downcast_ref::<u16>() {
            (*r as i64).into()
        } else if let Some(r) = r.downcast_ref::<u32>() {
            (*r as i64).into()
        } else if let Some(r) = r.downcast_ref::<u64>() {
            (*r as i64).into()
        } else if let Some(r) = r.downcast_ref::<isize>() {
            (*r as i64).into()
        } else if let Some(r) = r.downcast_ref::<usize>() {
            (*r as i64).into()
        } else {
            eprintln!("Can't process value at path {index}");
            return Err(Box::new(EvalAltResult::ErrorMismatchOutputType(
                index,
                "".to_string(),
                Position::NONE,
            )));
        };
        Ok(result)
    }
}

impl<T: UIState> From<&T> for UIStateHandle<T> {
    fn from(value: &T) -> Self {
        Self::new(value)
    }
}
