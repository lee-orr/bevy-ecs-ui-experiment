use crate::UIState;

use std::str::FromStr;

pub trait Expression<Val>: FromStr + Send + Sync + Clone {
    fn process<T: UIState>(&self, context: &T) -> Val;
}
