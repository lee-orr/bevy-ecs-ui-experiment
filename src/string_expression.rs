use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct StringExpression(Vec<(String, bool)>);

impl StringExpression {
    pub fn process<T: Serialize>(&self, context: &T) -> Option<String> {
        self.0.get(0).map(|b| b.0.clone())
    }

    pub fn process_unchecked<T: Serialize>(&self, context: &T) -> String {
        self.process(context).unwrap_or_default()
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
