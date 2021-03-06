use super::{DataSource, Produce, SourceBuilder};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use anyhow::anyhow;
use fehler::{throw, throws};
use num_traits::cast::FromPrimitive;

pub struct U64SourceBuilder {}

impl SourceBuilder for U64SourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = U64CounterSource;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::DataSource {
        U64CounterSource::new()
    }
}
/// This `DataSource` only produces T which can be derived from u64.
pub struct U64CounterSource {
    counter: u64,
    nrows: usize,
}

impl U64CounterSource {
    pub fn new() -> Self {
        Self {
            counter: 0,
            nrows: 0,
        }
    }
}

impl DataSource for U64CounterSource {
    type TypeSystem = DataType;

    // query: nrows
    fn run_query(&mut self, query: &str) -> Result<()> {
        self.nrows = query.parse().unwrap();
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.nrows
    }
}

impl Produce<u64> for U64CounterSource {
    fn produce(&mut self) -> Result<u64> {
        let ret = self.counter;
        self.counter += 1;
        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

impl Produce<f64> for U64CounterSource {
    fn produce(&mut self) -> Result<f64> {
        let ret = self.counter;
        self.counter += 1;
        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

impl Produce<Option<u64>> for U64CounterSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        let ret = self.counter;
        self.counter += 1;
        Ok(Some(FromPrimitive::from_u64(ret).unwrap_or_default()))
    }
}

impl Produce<String> for U64CounterSource {
    fn produce(&mut self) -> Result<String> {
        let ret = self.counter.to_string();
        self.counter += 1;
        Ok(ret)
    }
}

impl Produce<bool> for U64CounterSource {
    fn produce(&mut self) -> Result<bool> {
        let ret = self.counter % 2 == 0;
        self.counter += 1;
        Ok(ret)
    }
}

pub struct StringSourceBuilder {}

impl SourceBuilder for StringSourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = StringSource;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::DataSource {
        StringSource::new()
    }
}
/// This `DataSource` only produces T which can be derived from String

pub struct StringSource {
    rand_string: String,
    nrows: usize,
}

impl StringSource {
    pub fn new() -> Self {
        Self {
            rand_string: "0".to_string(),
            nrows: 0,
        }
    }
}

impl DataSource for StringSource {
    type TypeSystem = DataType;

    // query: nrows
    fn run_query(&mut self, query: &str) -> Result<()> {
        self.nrows = query.parse().unwrap();
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.nrows
    }
}

impl Produce<String> for StringSource {
    fn produce(&mut self) -> Result<String> {
        let ret = self.rand_string.clone();
        let new_val = ret.clone().parse::<u64>().unwrap() + 1;
        self.rand_string = new_val.to_string();

        Ok(ret)
    }
}

impl Produce<u64> for StringSource {
    fn produce(&mut self) -> Result<u64> {
        let ret = self.rand_string.clone().parse::<u64>().unwrap();
        let new_string = ret.clone() + 1;
        self.rand_string = new_string.to_string();

        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

impl Produce<Option<u64>> for StringSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        let ret = self.rand_string.clone().parse::<u64>().unwrap();
        let new_string = ret.clone() + 1;
        self.rand_string = new_string.to_string();

        Ok(Some(FromPrimitive::from_u64(ret).unwrap_or_default()))
    }
}

impl Produce<f64> for StringSource {
    fn produce(&mut self) -> Result<f64> {
        let ret = self.rand_string.clone().parse::<u64>().unwrap();
        let new_string = ret.clone() + 1;
        self.rand_string = new_string.to_string();

        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

impl Produce<bool> for StringSource {
    fn produce(&mut self) -> Result<bool> {
        throw!(anyhow!("StringSource only support string!"))
    }
}

pub struct BoolSourceBuilder {}

impl SourceBuilder for BoolSourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = BoolCounterSource;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::DataSource {
        BoolCounterSource::new()
    }
}
/// This `DataSource` only produces T which can be derived from bool.
pub struct BoolCounterSource {
    counter: bool,
    nrows: usize,
}

impl BoolCounterSource {
    pub fn new() -> Self {
        Self {
            counter: false,
            nrows: 0,
        }
    }
}

impl DataSource for BoolCounterSource {
    type TypeSystem = DataType;

    // query: nrows
    fn run_query(&mut self, query: &str) -> Result<()> {
        self.nrows = query.parse().unwrap();
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.nrows
    }
}

impl Produce<u64> for BoolCounterSource {
    fn produce(&mut self) -> Result<u64> {
        let ret = 1;
        self.counter = !self.counter;
        Ok(ret)
    }
}
impl Produce<Option<u64>> for BoolCounterSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        let ret = 1;
        self.counter = !self.counter;
        Ok(Some(ret))
    }
}
impl Produce<f64> for BoolCounterSource {
    fn produce(&mut self) -> Result<f64> {
        let ret = 1.0;
        self.counter = !self.counter;
        Ok(ret)
    }
}
impl Produce<bool> for BoolCounterSource {
    fn produce(&mut self) -> Result<bool> {
        let ret = self.counter;
        self.counter = !self.counter;
        Ok(ret)
    }
}

impl Produce<String> for BoolCounterSource {
    fn produce(&mut self) -> Result<String> {
        throw!(anyhow!("StringSource only support string!"))
    }
}
pub struct F64SourceBuilder {}

impl SourceBuilder for F64SourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = F64CounterSource;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::DataSource {
        F64CounterSource::new()
    }
}

/// This `DataSource` only produces T which can be derived from f64.
pub struct F64CounterSource {
    counter: f64,
    nrows: usize,
}

impl F64CounterSource {
    pub fn new() -> Self {
        Self {
            counter: 0.0,
            nrows: 0,
        }
    }
}

impl DataSource for F64CounterSource {
    type TypeSystem = DataType;

    fn run_query(&mut self, query: &str) -> Result<()> {
        self.nrows = query.parse().unwrap();
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.nrows
    }
}

impl Produce<u64> for F64CounterSource {
    fn produce(&mut self) -> Result<u64> {
        let ret = self.counter;
        self.counter += 0.5;
        Ok(FromPrimitive::from_f64(ret).unwrap_or_default())
    }
}

impl Produce<Option<u64>> for F64CounterSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        let ret = self.counter;
        self.counter += 0.5;
        Ok(Some(FromPrimitive::from_f64(ret).unwrap_or_default()))
    }
}

impl Produce<f64> for F64CounterSource {
    fn produce(&mut self) -> Result<f64> {
        let ret = self.counter;
        self.counter += 0.5;
        Ok(FromPrimitive::from_f64(ret).unwrap_or_default())
    }
}

impl Produce<bool> for F64CounterSource {
    fn produce(&mut self) -> Result<bool> {
        throw!(anyhow!("F64CounterSource only support f64!"))
    }
}

impl Produce<String> for F64CounterSource {
    fn produce(&mut self) -> Result<String> {
        throw!(anyhow!("F64CounterSource only support f64!"))
    }
}

pub struct OptU64SourceBuilder {
    fake_values: Vec<Vec<Option<u64>>>,
    ncols: usize,
}

impl OptU64SourceBuilder {
    pub fn new(fake_values: Vec<Vec<Option<u64>>>, ncols: usize) -> Self {
        OptU64SourceBuilder { fake_values, ncols }
    }
}

impl SourceBuilder for OptU64SourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = OptU64TestSource;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::DataSource {
        let ret = OptU64TestSource::new(self.fake_values.swap_remove(0), self.ncols);
        ret
    }
}

pub struct OptU64TestSource {
    counter: usize,
    vals: Vec<Option<u64>>,
    ncols: usize,
}

impl OptU64TestSource {
    pub fn new(vals: Vec<Option<u64>>, ncols: usize) -> Self {
        OptU64TestSource {
            counter: 0,
            vals: vals,
            ncols,
        }
    }
}

impl DataSource for OptU64TestSource {
    type TypeSystem = DataType;
    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.vals.len() / self.ncols
    }
}

impl Produce<u64> for OptU64TestSource {
    fn produce(&mut self) -> Result<u64> {
        let v = match self.vals[self.counter] {
            Some(v) => v,
            None => 0,
        };
        self.counter += 1;
        Ok(v)
    }
}

impl Produce<Option<u64>> for OptU64TestSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        let v = self.vals[self.counter];
        self.counter += 1;
        Ok(v)
    }
}

impl Produce<f64> for OptU64TestSource {
    fn produce(&mut self) -> Result<f64> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}

impl Produce<bool> for OptU64TestSource {
    fn produce(&mut self) -> Result<bool> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}

impl Produce<String> for OptU64TestSource {
    fn produce(&mut self) -> Result<String> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}
