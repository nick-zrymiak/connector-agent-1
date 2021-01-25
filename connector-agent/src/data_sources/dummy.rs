use super::{DataSource, Parse};
use crate::errors::Result;
use crate::types::DataType;
use num_traits::cast::FromPrimitive;

/// This `DataSource` only produces T which can be derived from u64.
pub struct U64CounterSource {
    counter: u64,
}

impl U64CounterSource {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl DataSource for U64CounterSource {
    type TypeSystem = DataType;

    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
    }
}

impl Parse<u64> for U64CounterSource {
    fn parse(&mut self) -> Result<u64> {
        let ret = self.counter;
        self.counter += 1;
        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

impl Parse<f64> for U64CounterSource {
    fn parse(&mut self) -> Result<f64> {
        let ret = self.counter;
        self.counter += 1;
        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

// String
pub struct StringSource {
    rand_string: String,
}

impl StringSource {
    pub fn new() -> Self {
        Self { rand_string: "a".to_string() }
    }
}

impl DataSource for StringSource {
    type TypeSystem = DataType;

    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
    }
}

impl Parse<String> for StringSource {
    fn parse(&mut self) -> Result<String> {
        let ret = self.rand_string.clone();

        let mut ascii_value = ret.as_bytes();
        let mut new_char = (ascii_value[0] + 1) as char;
        self.rand_string = new_char.to_string();

        Ok(ret)
    }
}
